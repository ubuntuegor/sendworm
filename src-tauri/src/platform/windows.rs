use windows::{
    core::*,
    ApplicationModel::Activation::{ActivationKind, ShareTargetActivatedEventArgs},
    ApplicationModel::AppInstance,
};

pub fn get_file_to_send_impl() -> Option<String> {
    match AppInstance::GetActivatedEventArgs() {
        Ok(args) => match args.Kind() {
            Ok(kind) => {
                if kind == ActivationKind::ShareTarget {
                    let share_args = args.cast::<ShareTargetActivatedEventArgs>().unwrap();
                    let operation = share_args.ShareOperation().unwrap();
                    operation.ReportStarted().unwrap();
                    let data = operation.Data().unwrap();
                    let items = data.GetStorageItemsAsync().unwrap().join().unwrap();
                    let result = match items.GetAt(0) {
                        Ok(item) => Some(item.Path().unwrap().to_string()),
                        Err(_) => None,
                    };
                    operation.ReportCompleted().unwrap();
                    result
                } else {
                    None
                }
            }
            Err(_) => None,
        },
        Err(_) => None,
    }
}
