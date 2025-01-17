use crate::services::field::CheckboxCellData;
use flowy_derive::{ProtoBuf, ProtoBuf_Enum};
use flowy_error::ErrorCode;
use flowy_grid_data_model::revision::GridFilterRevision;
use std::sync::Arc;

#[derive(Eq, PartialEq, ProtoBuf, Debug, Default, Clone)]
pub struct GridCheckboxFilter {
    #[pb(index = 1)]
    pub condition: CheckboxCondition,
}

impl GridCheckboxFilter {
    pub fn apply(&self, cell_data: &CheckboxCellData) -> bool {
        let is_check = cell_data.is_check();
        match self.condition {
            CheckboxCondition::IsChecked => is_check,
            CheckboxCondition::IsUnChecked => !is_check,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ProtoBuf_Enum)]
#[repr(u8)]
pub enum CheckboxCondition {
    IsChecked = 0,
    IsUnChecked = 1,
}

impl std::convert::From<CheckboxCondition> for i32 {
    fn from(value: CheckboxCondition) -> Self {
        value as i32
    }
}

impl std::default::Default for CheckboxCondition {
    fn default() -> Self {
        CheckboxCondition::IsChecked
    }
}

impl std::convert::TryFrom<u8> for CheckboxCondition {
    type Error = ErrorCode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CheckboxCondition::IsChecked),
            1 => Ok(CheckboxCondition::IsUnChecked),
            _ => Err(ErrorCode::InvalidData),
        }
    }
}

impl std::convert::From<Arc<GridFilterRevision>> for GridCheckboxFilter {
    fn from(rev: Arc<GridFilterRevision>) -> Self {
        GridCheckboxFilter {
            condition: CheckboxCondition::try_from(rev.condition).unwrap_or(CheckboxCondition::IsChecked),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::{CheckboxCondition, GridCheckboxFilter};
    use crate::services::field::CheckboxCellData;

    #[test]
    fn checkbox_filter_is_check_test() {
        let checkbox_filter = GridCheckboxFilter {
            condition: CheckboxCondition::IsChecked,
        };
        for (value, r) in [("true", true), ("yes", true), ("false", false), ("no", false)] {
            let data = CheckboxCellData(value.to_owned());
            assert_eq!(checkbox_filter.apply(&data), r);
        }
    }
}
