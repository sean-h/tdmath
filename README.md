# tdmath

tdmath is a 3D math library written in Rust. Its goal is to provide standard math structures and functions for 3D graphics applications such as games and renderers.

## Vector3
The Vector3 type is a 3D vector of f32 values.

```rust
// Vector3 constructor
let v = Vector3::new(1.0, 5.0, -6.0);
assert_eq!(v.x, 1.0);
assert_eq!(v.y, 5.0);
assert_eq!(v.z, -6.0);

// Array notation
assert_eq!(v.x, v[0]);
assert_eq!(v.y, v[1]);
assert_eq!(v.z, v[2]);

// Normalization
let n = v.normalized();
assert_eq!(n.length(), 1.0);

// Dot and Cross products
let v1 = Vector3::new(1.0, 5.0, -6.0);
let v2 = Vector3::new(0.0, 1.0, 1.0);
let dot = Vector3::dot(v1, v2);
let cross = Vector3::cross(v1, v2);

// Math operations
let v3 = v1 + v2;
let v4 = 3.0 * v1;

```

## Matrix4
The Matrix4 type is a 4x4 matrix of f32 values.

```rust
let translation = Matrix4::translation(1.0, 0.0, 5.0);
let rotation = Matrix4::rotation(Quaternion::new(1.0, 0.0, 0.0));
let scale = Matrix4::scale(1.0, 5.0, 1.0);
let v = Vector3::new(5.0, 5.0, 20.0);

let v2 = scale * rotation * translation * v;
```

## Quaternion
The Quaternion type is a collection of 4 f32 values (x, y, z, w) for expressing rotations.

```rust
// Parameters are in radians
let q = Quaternion::new(1.0, 0.0, -1.4);
```

## Ray
The Ray type is a collection of origin, direction and time.

```rust
let r = Ray::new(Vector3::zero(), Vector3::forward(), 0.0);
let origin = r.origin();
let direction = r.direction();
```
