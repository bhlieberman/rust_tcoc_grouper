
struct HCPS_code{
    units:i32,
    revenue_Code:i32,
    mods:vec<String>,
    charge:f32,
    short_description:String,
    long_description:String

};

impl HCPS_code{
    fn new(un:i32,rc:132,m:vec<String>,charg:f32,short:&str,long:&str)->HCPS_code{
        HCPS_code{
            units:un,
            revenue_Code:rc,
            mods:&m,
            short_description:short.to_string(),
            long_description: long.to_string()
        }

    }
    
}