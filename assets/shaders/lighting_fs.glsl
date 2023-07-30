#version 400
in vec2 position_pass;
out vec4 final_color;

uniform sampler3D world_data_texture;
uniform sampler2D previous_position_texture;
uniform sampler2D previous_lighting_texture;
uniform sampler2D current_position_texture;

uniform mat4 previous_mvp;
uniform float time;

const float WORLD_SIZE = 256;
const float VOXEL_SIZE = 0.01;
vec3 SUN_DIRECTION = normalize(vec3(-0.2, -0.8, -0.2));
const float SUN_SIZE = 0.60;
const int NBR_OF_REBOUNDS = 1; 
const float offset_lighting = 0.15;

///////////////////// STRUCTS
struct PointLight{
    vec3 position;
    vec3 color;
    float intensity;
};

struct Surface{
    vec3 position;
    vec3 normal;
};


///////////////////// UTILS
vec3 random_vec3(vec3 position){
    float x = sin(dot(position, vec3(12.9898, 78.233, 45.5432))) * 3452.5453;
    float y = sin(dot(position, vec3(93.989, 67.345, 23.123))) * 3452.9483;
    float z = sin(dot(position, vec3(43.332, 93.532, 63.121))) * 3452.4567;
    return normalize(vec3(x, y, z));
}

float random_float(vec3 position){
    return fract(sin(position.x) * 35862.256 - cos(position.y) * 25789.536 + sin(position.z) * 15488.5986 + 58488.0256 * cos(time));
}

vec3 mod_3d(vec3 position, float value){
    return vec3(mod(position.x, value), mod(position.y, value), mod(position.z, value));
}
///////////////////// 

int get_id_cube(vec3 position){
    vec3 position_in_texture = floor(position/VOXEL_SIZE)/WORLD_SIZE;
    if (position_in_texture.x < 0 || position_in_texture.y < 0 || position_in_texture.z < 0 || position_in_texture.x > 1 || position_in_texture.y > 1 || position_in_texture.z > 1){
        return 0;
    }
    int bloc_id = int(255.0 * texture(world_data_texture, position_in_texture).r);
    return bloc_id;
}

bool is_cube(vec3 position){
    return get_id_cube(position) > 0;
}

float distance_to_border(vec3 position , vec3 direction){
    float minimum_x = max((1.0001 - fract(position.x/VOXEL_SIZE)) / direction.x, (-0.0001 - fract(position.x/VOXEL_SIZE)) / direction.x);
    float minimum_y = max((1.0001 - fract(position.y/VOXEL_SIZE)) / direction.y, (-0.0001 - fract(position.y/VOXEL_SIZE)) / direction.y);
    float minimum_z = max((1.0001 - fract(position.z/VOXEL_SIZE)) / direction.z, (-0.0001 - fract(position.z/VOXEL_SIZE)) / direction.z);

    return max(0.01 * VOXEL_SIZE, VOXEL_SIZE * min(minimum_x, min(minimum_y, minimum_z)));
}

bool is_out_of_map(vec3 position){
    return position.x < 0.0 || position.y < 0.0 || position.z < 0.0 || position.x > VOXEL_SIZE * WORLD_SIZE || position.y > VOXEL_SIZE * WORLD_SIZE || position.z > VOXEL_SIZE * WORLD_SIZE;
}

vec3 get_normal(vec3 position){
    vec3 center = VOXEL_SIZE * (floor(position / VOXEL_SIZE) + vec3(0.5, 0.5, 0.5));
    if(abs(center.x - position.x) > abs(center.y - position.y) && abs(center.x - position.x) > abs(center.z - position.z)){
        if(center.x - position.x > 0.0){
            return vec3(-1.0, 0.0, 0.0);
        }
        return vec3(1.0, 0.0, 0.0);
    }
    if(abs(center.y - position.y) > abs(center.z - position.z)){
        if(center.y - position.y > 0.0){
            return vec3(0.0, -1.0, 0.0);
        }
        return vec3(0.0, 1.0, 0.0);
    }
    if(center.z - position.z > 0.0){
        return vec3(0.0, 0.0, -1.0);
    }
    return vec3(0.0, 0.0, 1.0);
}

vec2 get_texture_coord_previous_position(vec3 position, vec3 normal){
    vec4 previous_position = previous_mvp * vec4(position, 1);
    return 0.5 * (vec2(1.0) + vec2(previous_position.x / previous_position.w, previous_position.y / previous_position.w));
    
}

vec3 calculate_direct_lighting(const Surface surface, const PointLight light){
    // Calculez la direction de la lumière incidente et la distance
    vec3 lightDirection = normalize(light.position - surface.position);
    float distance_to_light = length(light.position - surface.position);
    
    // Calculez l'atténuation de la lumière en fonction de la distance
    float attenuation = 1.0;

    // Calculate random vector in hemisphere
    vec3 random_direction = random_vec3(mod_3d(7244.57 * surface.position, 145.45) * mod(54.78 * time, 28.540));
    random_direction = normalize(random_direction * sign(dot(surface.normal, random_direction)));
    
    // Calculez le produit scalaire entre la direction de la lumière et la normale de la surface
    float NdotL = max(dot(surface.normal, lightDirection), 0.0);

    // Calculez le produit scalaire entre la direction de la lumière et l'échantillon aléatoire
    float NdotS = max(dot(surface.normal, random_direction), 0.0);
    
    // Calculez la contribution de la source de lumière
    vec3 directLighting = light.color * light.intensity * attenuation * NdotL * NdotS;

    //directLighting = vec3(pow(min(directLighting.x, 0.9), 0.2), pow(min(directLighting.y, 0.9), 0.2), pow(min(directLighting.z, 0.9), 0.2));

    
    return directLighting;
}

vec3 get_sun_illumination(vec3 start_position, vec3 normal){
    // if (dot(normal, -SUN_DIRECTION) < 0.0){
    //     return vec3(0.0);
    // }
    // vec3 ray_forward = normalize(-SUN_DIRECTION);
    // vec3 ray_position = start_position + 1.02 * distance_to_border(start_position, ray_forward) * ray_forward;
    
    // while(length(ray_position - start_position) < 6.0 && !is_out_of_map(ray_position)){
    //     // If there is a cube : obstruction of light by sun
    //     int bloc_id = get_id_cube(ray_position);
    //     if (bloc_id > 0){
    //         return vec3(0.0);
    //     }       

    //     ray_position += distance_to_border(ray_position, ray_forward) * ray_forward;        
    // }

    // // No bloc has been touched, if the ray is going in the direction of the sun : consider it lighted
    // if (dot(ray_forward, -SUN_DIRECTION) > SUN_SIZE){ 
    //     return vec3(1.0);
    // }

    return vec3(1.0);
}

vec3 get_light_illumination(vec3 start_position, vec3 normal, int ray_index){
    vec3 random_direction = random_vec3(mod_3d(7244.57 * start_position, 145.45) * mod(54.78 * time, 28.540) * (mod(float(ray_index) * 3.72, 3.268) + 1));
    vec3 ray_forward = normalize(random_direction * sign(dot(normal, random_direction)));
    vec3 ray_position = start_position +  distance_to_border(start_position, ray_forward) * ray_forward;
    
    while(length(ray_position - start_position) < 0.1 && !is_out_of_map(ray_position)){
        // If there is a cube : obstruction of light
        int bloc_id = get_id_cube(ray_position);
        if (bloc_id > 0.5){
            return vec3(1.0 - (1.0 / (1.0 + 0.05 * length(ray_position - start_position))));
        }
        ray_position += distance_to_border(ray_position, ray_forward) * ray_forward;        
    }

    // No bloc has been touched
    return vec3(1.0 - (1.0 / (1.0 + 0.05 * 0.1)));
}



void main()
{
    // Find origin position and direction of ray
    // Direction is computed using MVP matrix
    vec4 current_position_texture = texture(current_position_texture, 0.5 * (position_pass + vec2(1.0)));
    vec3 point_position = current_position_texture.xyz/current_position_texture.w;

    vec3 current_color = vec3(0.0);

    SUN_DIRECTION = normalize(vec3(cos(time * 0.01), -1.8, sin(time * 0.01)));
    
    // If there is a cube at this position
    if (current_position_texture.a > 0.5){
        vec3 normal = get_normal(point_position);
        vec3 current_illumination = (1.0 * get_sun_illumination(point_position, normal) + vec3(offset_lighting)) * get_light_illumination(point_position, normal, 0);
        vec2 text_coord_previous = get_texture_coord_previous_position(point_position, normal);
        // If this pixel was not out of the screen the previous frame: reuse previous image
        if (text_coord_previous.x > 0 && text_coord_previous.y > 0 && text_coord_previous.x < 1.0 && text_coord_previous.y < 1.0){
            vec4 previous_position_texture = texture(previous_position_texture, text_coord_previous);
            vec3 previous_position = previous_position_texture.rgb;
            float length_position_delta = length(previous_position - point_position);
            // If the two point are the same
            if ((length_position_delta < 0.2 * VOXEL_SIZE /*&& dot(normal, get_normal(previous_position)) > 0.8*/) /* || length_position_delta < 0.01 * VOXEL_SIZE*/) {
                vec4 previous_illumination_texture = texture(previous_lighting_texture, text_coord_previous);
                vec3 previous_illumination = previous_illumination_texture.rgb; 
                vec3 value_after_coef = (previous_illumination * previous_illumination_texture.a + current_illumination) / (previous_illumination_texture.a + 1.0);
                final_color = vec4(value_after_coef, previous_illumination_texture.a + 1.0);
                if (previous_illumination_texture.a > 100.0){
                    final_color.a = 100.0;
                }
                return;
            }
        }
         // New point ==> use X rays to estimate the light
        int number_of_ray = 5;
        vec3 value_acc = current_illumination;
        vec3 sun_illumination = (1.0 * get_sun_illumination(point_position, normal) + vec3(offset_lighting));
        for(int i = 1; i < number_of_ray; i++){
            value_acc += sun_illumination * get_light_illumination(point_position, normal, i);
        }
        final_color = vec4(value_acc / float(number_of_ray), float(number_of_ray));
        return;
    }

    final_color = vec4(0.0, 0.0, 0.0, 0.0);
}

