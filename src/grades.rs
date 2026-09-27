pub struct Year {
    pub name: String, 
    subjects: Vec<Subject>,
    avg: f32, 
}

pub struct Subject{
    name: String, 
    ects: u8, 
    simul: bool, 
    avg: u8,
}

pub fn create_year(year: String)-> Year {
    let subs: Vec<Subject> = Vec::new(); 
    Year {name: year, subjects: subs, avg: 0.0}
}

pub fn total_avg(years: Vec<Year>) -> f32 {
    let mut average: f32 = 0.0; 
    let mut year_count: u8 = 0; 
    for item in years {
        average += item.avg; 
        year_count += 1; 
    }
    if year_count == 0 {
        return 0.0
    }
    average / year_count as f32
}

impl Year {

    pub fn create_subject(&mut self, subject: String, credits: u8, sim: bool, grade: u8) -> () {
        self.subjects.push(Subject {name: subject, ects: credits, simul: sim, avg: grade}); 
    } 

    pub fn edit_grade(&mut self, subject: String, grade: u8) -> () {
        for item in &mut self.subjects {
            if item.name == subject {
                item.avg = grade;
            }
        }
    }

    pub fn edit_sim(&mut self, subject: String, sim: bool) -> () {
        for item in &mut self.subjects {
            if item.name == subject {
                item.simul = sim;
            }
        }
    }

    pub fn delete_subject(&mut self, subject: String) -> () {
        self.subjects.retain(|item| item.name != subject);
    }

    pub fn yearly_average(&mut self) -> f32 {
        let mut sum = 0; 
        let mut credits = 0; 
        let average; 
        for item in &self.subjects {
            sum += item.avg * item.ects; 
            credits += item.ects; 
        }
        if credits == 0 {
            return 0.0
        }
        average = sum as f32 / credits as f32; 
        self.avg = average;
        average
    }

    pub fn print_year(&mut self) -> () {
        println!("=== {} ===", self.name); 
        for item in &self.subjects {
            println!("{} | Grade: {} | Sim: {} | ECTS: {}", item.name, item.avg, if item.simul == true {"S"} else {"T"}, item.ects); 
        }
        println!("The average this year was {}", self.yearly_average()); 
        println!("==================="); 
    }
    
}

