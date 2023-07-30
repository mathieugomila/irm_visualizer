#version 400
in vec2 position_pass;
out vec4 final_color;

uniform sampler3D world_data_texture;

uniform mat4 invert_mvp;
uniform vec3 camera_position;

const float WORLD_SIZE = 256;
const float VOXEL_SIZE = 0.01;


bool is_cube(vec3 position){
    vec3 position_in_texture = floor(position/VOXEL_SIZE)/WORLD_SIZE;
    if (position_in_texture.x < 0 || position_in_texture.y < 0 || position_in_texture.z < 0 || position_in_texture.x > 1 || position_in_texture.y > 1 || position_in_texture.z > 1){
        return false;
    }
    if (texture(world_data_texture, position_in_texture).r > 0.0){
        return true;
    }
    return false;
}

float distance_to_border(vec3 position , vec3 direction){
    float minimum_x = max((1.0001 - fract(position.x/VOXEL_SIZE)) / direction.x, (-0.0001 - fract(position.x/VOXEL_SIZE)) / direction.x);
    float minimum_y = max((1.0001 - fract(position.y/VOXEL_SIZE)) / direction.y, (-0.0001 - fract(position.y/VOXEL_SIZE)) / direction.y);
    float minimum_z = max((1.0001 - fract(position.z/VOXEL_SIZE)) / direction.z, (-0.0001 - fract(position.z/VOXEL_SIZE)) / direction.z);

    return VOXEL_SIZE * min(minimum_x, min(minimum_y, minimum_z));
}

bool is_out_of_map(vec3 position){
    return position.x < 0.0 || position.y < 0.0 || position.z < 0.0 || position.x > VOXEL_SIZE * WORLD_SIZE || position.y > VOXEL_SIZE * WORLD_SIZE || position.z > VOXEL_SIZE * WORLD_SIZE;
}

float distance_to_grid(vec3 position, float grid_space, float grid_radius){
    if (is_out_of_map(position)){
        return 100.0;
    }

    float grid_y = max(abs(mod(position.x - grid_radius/2.0 - VOXEL_SIZE, grid_space) - grid_space), abs(mod(position.z - VOXEL_SIZE - grid_radius/2.0, grid_space) - grid_space)); 
    float grid_z = max(abs(mod(position.x - grid_radius/2.0 - VOXEL_SIZE, grid_space) - grid_space), abs(mod(position.y - VOXEL_SIZE - grid_radius/2.0, grid_space) - grid_space)); 
    float grid_x = max(abs(mod(position.y - grid_radius/2.0 - VOXEL_SIZE, grid_space) - grid_space), abs(mod(position.z - VOXEL_SIZE - grid_radius/2.0, grid_space) - grid_space)); 

    return min(min(grid_x, grid_y), grid_z);
}

void main()
{
    bool ray_entered_world = false;
    vec3 border_first_position = vec3(0.0);
    // Find origin position and direction of ray
    // Direction is computed using MVP matrix
    vec3 ray_position = camera_position;
    vec3 ray_forward = normalize(vec4(invert_mvp * vec4(position_pass, 0.001, 1.0)).xyz);

    while(length(ray_position - camera_position) < 5.0){
        if (!is_out_of_map(ray_position)) {
            ray_entered_world = true;
        }

        if (is_out_of_map(ray_position) && ray_entered_world){
            final_color = vec4(ray_position, 0.25);
            return;
        }

        if (!is_out_of_map(ray_position) && is_cube(ray_position)){
            final_color = vec4(ray_position, 1.0);
            return;
        }
        
        float grid_space = 0.1;
        float grid_radius = 0.001;
        if (distance_to_grid(ray_position, grid_space, grid_radius) < grid_radius){
            final_color = vec4(ray_position, 1.0);
            return;
        }
       
        ray_position += distance_to_border(ray_position, ray_forward) * ray_forward;
        
      
    }

    final_color = vec4(ray_position, 0.25);
}
