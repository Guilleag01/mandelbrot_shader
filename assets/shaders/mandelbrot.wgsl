struct RenderParams {
    width_pixels: f64,
    height_pixels: f64,
    width: f64,
    height: f64,
    offset_x: f64,
    offset_y: f64,
    iters: f64,
};

@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

// const iters: i32 = 100;
const lim: f64 = 1000;

@group(0) @binding(1) var<uniform> render_params: RenderParams;

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    // let location_f64 = vec2(f64(location[0]), f64(location[1]));
    var color = vec4<f32>(0.0, 1.0, 0.0, 1.0);

    if invocation_id.x > 600 {
        color = vec4<f32>(0.0, 0.0, 1.0, 1.0);
    }
    textureStore(texture, location, color);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    let location_f64 = vec2(f64(location[0]), f64(location[1]));

    let location_centered = vec2((location_f64[0] - render_params.width_pixels / 2.0),
        (location_f64[1] - render_params.height_pixels / 2.0));

    let pos = vec2((location_centered[0] * (render_params.width / 2.0) / (render_params.width_pixels / 2.0) + render_params.offset_x),
        (location_centered[1] * (render_params.height / 2.0) / (render_params.height_pixels / 2.0) + render_params.offset_y));

    let v: i32 = goes_to_infinite(pos);
    let color = scalar_to_rainbow(f32(v) / f32(render_params.iters)); //vec4<f32>(0.0, f32(v) / f32(render_params.iters), 0.0, 1.0);
    textureStore(texture, location, color);
}

fn sum_complex(a: vec2<f64>, b: vec2<f64>) -> vec2<f64> {
    return vec2(a[0] + b[0], a[1] + b[1]);
}

fn square_complex(c: vec2<f64>) -> vec2<f64> {
    return vec2(c[0] * c[0] - c[1] * c[1], c[0] * c[1] + c[1] * c[0]);
}

fn f(z: vec2<f64>, c: vec2<f64>) -> vec2<f64> {
    return sum_complex(square_complex(z), c);
}

fn goes_to_infinite(c: vec2<f64>) -> i32 {
    // let iters: i32 = 200;
    // let lim: f64 = f64(100000);
    var z: vec2<f64> = vec2(0.0, 0.0);

    for (var i: i32 = 0; i < i32(render_params.iters); i += 1) {
        z = f(z, c);
        if (z[0] > lim) || (z[1] > lim) {
            return i;
        }
    }
    return i32(render_params.iters);
}

fn scalar_to_rainbow(val: f32) -> vec4<f32> {
    // let rgba = vec4(0.0, 0.0, 0.0, 0.0);

    var a = (1.0 - val) * 5;
    var X = floor(a);
    var Y = floor(255.0 * (a - X));

    switch(i32(X))
    {
        case 0: {
            return vec4(1.0, (Y / 255.0), 0.0, 1.0);
        }
        case 1: {
            return vec4(1.0 - (Y / 255.0), 0.0, 0.0, 1.0);
        }
        case 2: {
            return vec4(0.0, 1.0, (Y / 255.0), 1.0);
        }
        case 3: {
            return vec4(0.0, 1.0 - (Y / 255.0), 1.0, 1.0);
        }
        case 4: {
            return vec4((Y / 255.0), 0.0, 1.0, 1.0);
        }
        case 5: {
            return vec4(1.0, 0.0, 1.0, 1.0);
        }
        default: {
            return vec4(0.0, 0.0, 0.0, 1.0);
        }
        // case 0: {r = 255;g = Y;b = 0;break;
        //     case 1: r = 255 - Y;g = 255;b = 0;break;
        //     case 2: r = 0;g = 255;b = Y;break;
        //     case 3: r = 0;g = 255 - Y;b = 255;break;
        //     case 4: r = Y;g = 0;b = 255;break;
        //     case 5: r = 255;g = 0;b = 255;break;
    }
}