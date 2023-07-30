#version 400
in vec2 position_pass;
out vec4 final_color;

uniform sampler3D world_data_texture;
uniform sampler2D current_lighting_texture;
uniform sampler2D current_position_texture;
uniform float time;


const float WORLD_SIZE = 256;
const float VOXEL_SIZE = 0.01;
const vec2 window_size = vec2(1024.0, 768.0);
const vec2 delta = vec2(1.0 / window_size.x, 1.0 / window_size.y);
const float LIGHT_DIMINUTION = 15.0;


vec4 get_texture(vec3 ray_position){
    vec3 position_in_texture = floor(ray_position/VOXEL_SIZE)/WORLD_SIZE;
    return texture(world_data_texture, position_in_texture);
}

int get_id_cube(vec3 ray_position){
    return int(255.0 * get_texture(ray_position).r);
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

const float KERNEL_SIZE = 5.0; // 5
const float SIGMA_SPACE = 0.1; // 0.5
const float SIGMA_RANGE = 0.2; // 20
const float M_PI = 3.1415;

float gaussian(float x, float sigma) {
    return exp(-(x * x) / (2.0 * sigma * sigma)) / (sigma * sqrt(2.0 * M_PI));
}

vec3 apply_bilateral_filter(vec2 uv) {
    
    vec2 texelSize = 2.0 / vec2(textureSize(current_lighting_texture, 0));

    vec3 centralColor = texture2D(current_lighting_texture, uv).rgb;
    //centralColor = vec3(pow(min(centralColor.x, 0.9), 0.2), pow(min(centralColor.y, 0.9), 0.2), pow(min(centralColor.z, 0.9), 0.2));
    vec3 centralPosition = texture2D(current_position_texture, uv).rgb;
    vec3 centralNormal = get_normal(centralPosition);

    vec3 result = vec3(0.0);
    float totalWeight = 0.0;

    for (float x = -KERNEL_SIZE; x <= KERNEL_SIZE; x++) {
        for (float y = -KERNEL_SIZE; y <= KERNEL_SIZE; y++) {
            vec2 offset = vec2(x, y) * texelSize;
            vec2 sampleUV = uv + offset;

            vec3 sampleColor = texture2D(current_lighting_texture, sampleUV).rgb;
            //sampleColor = vec3(pow(min(sampleColor.x, 0.9), 0.2), pow(min(sampleColor.y, 0.9), 0.2), pow(min(sampleColor.z, 0.9), 0.2));
            vec3 samplePosition = texture2D(current_position_texture, sampleUV).rgb;
            vec3 sampleNormal = get_normal(samplePosition);

            float spaceDistance = length(offset);
            float rangeDistance = length(centralColor - sampleColor);
            float positionDistance = length(centralPosition - samplePosition);
            float normalDistance = length(centralNormal - sampleNormal);

            float spaceWeight = gaussian(spaceDistance, SIGMA_SPACE);
            float rangeWeight = gaussian(rangeDistance, SIGMA_RANGE);
            float positionWeight = gaussian(positionDistance, SIGMA_SPACE);
            float normalWeight = gaussian(normalDistance, SIGMA_SPACE);

            float weight = spaceWeight * rangeWeight * positionWeight * normalWeight;

            result += sampleColor * weight;
            totalWeight += weight;
        }
    }

    return result / totalWeight;
}


float smooth_float(float value){
    float value_1 = min(1.0, 1.0 * (2.0/(1.0 + exp(-LIGHT_DIMINUTION * value)) - 1.0));
    return 20 * value_1;
}


void main()
{
    // Add a cursor
    if (length(position_pass / delta) < 10.0){
        final_color = vec4(1.0, 0.0, 0.0, 1.0);
        return;
    }

    vec2 uv = 0.5 * (position_pass + vec2(1.0));

    vec4 current_position_texture_value = texture(current_position_texture, uv);
    if (current_position_texture_value.a > 0.5){
        vec3 filtered_light = texture(current_lighting_texture, uv).rgb;
        float pow_factor = 0.6;
        filtered_light = vec3(smooth_float(filtered_light.x), smooth_float(filtered_light.y), smooth_float(filtered_light.z));
        int bloc_id = get_id_cube(current_position_texture_value.xyz);

        if (bloc_id == 1){
            vec3 color = vec3(0.6, 0.4, 0.15);
            final_color = vec4(filtered_light * color, 1.0);
        }

        else if (bloc_id == 2){
            vec3 color = vec3(0.0, 1.0, 0.0);
            final_color = vec4(filtered_light * color, 1.0);
        }

        else if (bloc_id == 3){
            vec3 color = vec3(1.0, 1.0, 1.0);
            final_color = vec4(filtered_light * color, 1.0);
        }

        else if (bloc_id == 4){
            vec3 color = vec3(1.0, 1.0, 0.0);
            final_color = vec4(filtered_light * color, 1.0);
        }

        return;
    }
}

