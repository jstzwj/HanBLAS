use super::util::sign;

pub fn srotg(sa: &mut f32, sb: &mut f32, c: &mut f32, s: &mut f32) {
    let mut roe: f32 = *sb;
    if sa.abs() > sb.abs() {
        roe = *sa;
    }
    let scale = sa.abs() + sb.abs();
    let (mut r, mut z);
    if scale == 0.0f32 {
        *c = 1.0f32;
        *s = 0.0f32;
        r = 0.0f32;
        z = 0.0f32;
    } else {
        r = scale*((*sa/scale).powi(2)+ (*sb/scale).powi(2)).sqrt();
        r = sign(&1.0f32, &roe)*r;
        *c = *sa / r;
        *s = *sb / r;
        z = 1.0f32;
        if sa.abs() > sb.abs()
        {
            z = *s;
        }
        if sb.abs() >= sa.abs() && *c != 0.0f32
        {
            z = 1.0f32 / *c
        }
    }
    *sa = r;
    *sb = z;
}