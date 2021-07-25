
// انشاء ستراكت
struct Animal{
    animal_type: String,
    name: String,
    age: u8,
    color: String,
}

// انشاء دوال الستراكت اعلاه
impl Animal {
    // تستخدم الدالة لانشاء كائن من الاستراكت
    fn new(animal_type: &str, name: &str, age: u8, color: &str) -> Animal {
        Animal {
            animal_type: String::from(animal_type).to_ascii_lowercase(), 
            name: String::from(name).to_ascii_lowercase(), 
            age: age, 
            color: String::from(color).to_ascii_lowercase()
        }
    }
    // تستخدم لجعل الكائن يقول نص معين
    fn say(&self, text: String) {
        println!("{}, {}", 
            self.bio(), text);
    }
    // ارجاع معلومات الكائن
    fn bio(&self) -> String {
        format!("
        \rاهلا انا {}, اسمي {} وعمري {}، لوني {}",
        self.animal_type, self.name, self.str_age(), self.color)
    }
    // تحويل العمر من رقم الى سنين
    fn str_age(&self) -> String {
        /*
        عندما يكون العمر اقل او يساوي 10 سنين ولا يساوي واحد سوف يتم ارجاع (<عمر الكائن> سنين) واذا يساوي واحد
        سوف يتم ارجاع (سنة واحدة) 
        واذا كان اكبر من ال 10 سوف يتم ارجاع (<عمر الكائن> سنة)
        */
        if self.age <= 10 { 
                if self.age != 1 { format!("{} سنين", self.age) } 
                else { String::from("سنة واحدة") } } 
        else { format!("{} سنة",self.age) }
    }
}


fn main() {
    // تعريف ثلاث كائنات واسناد قيمة مختلفة لكل كائن
    let dog:Animal = Animal::new("كلب", "فانسكورس", 1, "ابيض");
    let monkey: Animal = Animal::new("قرد", "قردردو", 5, "بني");
    let cat: Animal = Animal::new("قطة", "كاتي", 11, "مائل الى الصفار");
    // استخدام احد دوال الكائنات لجعلم يقولو نصوص معينة
    dog.say(format!("يعجبني اسمي {} اضنه جميل", dog.name));
    monkey.say(String::from("احب ان العب كثيرا"));
    cat.say(format!("عمري {} اضن انيي كبيرة", cat.str_age()))

}
