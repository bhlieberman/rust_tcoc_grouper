//not sure about storing date as string. saw you using chron::datettime
struct procedure_code{
    date:String,
    code_value:String
};

impl procedure_code(d:&str,cd:&str)->procedure_code{

    procedure_code{
        date = d.to_string(),
        code_value = cd.to_string()
    }

}