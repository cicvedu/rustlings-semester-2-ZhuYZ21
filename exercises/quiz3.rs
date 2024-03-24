pub trait Grade {
    fn grade(&self) -> String;
}

impl Grade for f32 {
    fn grade(&self) -> String {
        format!("{:.1}", self)
    }
}

impl Grade for String {
    fn grade(&self) -> String {
        self.clone()
    }
}

pub struct ReportCard<G: Grade> {
    pub grade: G,
    pub student_name: String,
    pub student_age: u8,
}

impl<G: Grade> ReportCard<G> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            self.grade.grade()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
