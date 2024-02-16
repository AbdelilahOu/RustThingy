pub use super::activities::{ActiveModel as ActivityActiveModel, Entity as Activity};
pub use super::cities::{ActiveModel as CityActiveModel, Entity as City};
pub use super::classes::{ActiveModel as ClassActiveModel, Entity as Class};
pub use super::countries::{ActiveModel as CountryActiveModel, Entity as Country};
pub use super::districts::{ActiveModel as DistrictActiveModel, Entity as District};
pub use super::events::{ActiveModel as EventActiveModel, Entity as Event};
pub use super::groups::{ActiveModel as GroupActiveModel, Entity as Group};
pub use super::lectures::{ActiveModel as LectureActiveModel, Entity as Lecture};
pub use super::levels::{ActiveModel as LevelActiveModel, Entity as Level};
pub use super::parents::{ActiveModel as ParentActiveModel, Entity as Parent};
pub use super::person_details::{ActiveModel as PersonDetailsActiveModel, Entity as PersonDetails};
pub use super::persons::{ActiveModel as PersonActiveModel, Entity as Persons};
pub use super::pickups::{ActiveModel as PickupActiveModel, Entity as Pickups};
pub use super::rooms::{ActiveModel as RoomActiveModel, Entity as Room};
pub use super::scans::{ActiveModel as ScanActiveModel, Entity as Scans};
pub use super::states::{ActiveModel as StateActiveModel, Entity as State};
pub use super::streets::{ActiveModel as StreetActiveModel, Entity as Street};
pub use super::students::{ActiveModel as StudentActiveModel, Entity as Student};
pub use super::subjects::{ActiveModel as SubjectActiveModel, Entity as Subject};
pub use super::teacher_subjects::{
    ActiveModel as TeacherSubjectActiveModel, Entity as TeacherSubject,
};
pub use super::teachers::{ActiveModel as TeacherActiveModel, Entity as Teacher};
pub use super::time_table::{ActiveModel as TimeTableActiveModel, Entity as TimeTable};
pub use super::users::{self, ActiveModel as UserActiveModel, Entity as User};
