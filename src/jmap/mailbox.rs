use serde::{Deserialize, Serialize};

use crate::jmap::Id;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Mailbox {
	id:             Id,
	name:           String,
	parent_id:      Option<Id>,
	role:           Option<String>,
	sort_order:     u64,
	total_emails:   u64,
	unread_emails:  u64,
	total_threads:  u64,
	unread_threads: u64,
	my_rights:      MailboxRights,
	is_subscribed:  bool,
	// "hidden": 0,
	// "purgeOlderThanDays": 31,
	// "identityRef": null,
	// "autoPurge": false,
	// "sort": [
	// {
	// "isAscending": false,
	// "property": "receivedAt"
	// }
	// ],
	// "learnAsSpam": false,
	// "suppressDuplicates": true,
	// "isCollapsed": false,
	// "autoLearn": false
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct MailboxRights {
	may_read_items:   bool,
	may_add_items:    bool,
	may_remove_items: bool,
	may_set_seen:     bool,
	may_set_keywords: bool,
	may_create_child: bool,
	may_rename:       bool,
	may_delete:       bool,
	may_submit:       bool,
}
