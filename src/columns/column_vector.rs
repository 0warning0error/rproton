use super::IColumn;
use super::{StringRef,Field};
use core::fmt::Debug;

#[derive(Debug)]
pub struct ColumnVector<T> {
    data: Vec<T>,
}

impl<T> ColumnVector<T>
where
    T: Copy + Debug + 'static,
{
    pub fn new(data: Vec<T>) -> Self {
        ColumnVector { data }
    }
}

impl<T> IColumn for ColumnVector<T>
where
    T: Copy + Debug + 'static,
{
    fn clone_box(&self) -> Box<dyn IColumn> {
        Box::new(ColumnVector {
            data: self.data.clone(),
        })
    }

    fn get_family_name(&self) -> &str {
        "ColumnVector"
    }

    fn get_data_type(&self) -> &str {
        std::any::type_name::<T>()
    }

    fn convert_to_full_column_if_const(&self) -> Box<dyn IColumn> {
        Box::new(ColumnVector {
            data: self.data.clone(),
        })
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn get_field(&self, n: usize) -> Field {
        // Implementation for retrieving a field
        Field
    }

    fn get(&self, n: usize, res: &mut Field) {
        // Implementation for setting a field
    }

    fn get_data_at(&self, n: usize) -> StringRef {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &self.data[n] as *const T as *const u8,
                std::mem::size_of::<T>(),
            )
        };
        StringRef::new(bytes)
    }
}

#[test]
fn test_column_vector() {
    let col = ColumnVector::new(vec![1, 2, 3, 4]);
    let col_box: Box<dyn IColumn> = Box::new(col);

    println!("Column family name: {}", col_box.get_family_name());
    println!("Column size: {}", col_box.size());
    let data_ref = col_box.get_data_at(1);
    println!("Data at index 1: {:?}", data_ref.data);
}