use crate::gui::chats::ChatList;

#[derive(Debug, Default)]
pub(crate) struct TrackedWidgets {
    lbl_listener_status: Option<gtk::Label>,
    chat_list: Option<ChatList>,
}

impl TrackedWidgets {
    pub(crate) fn lbl_listener_status(&self) -> Option<&gtk::Label> {
        self.lbl_listener_status.as_ref()
    }

    pub(crate) fn set_lbl_listener_status(&mut self, lbl_listener_status: Option<gtk::Label>) {
        self.lbl_listener_status = lbl_listener_status;
    }

    pub(crate) fn chat_list(&self) -> Option<&ChatList> {
        self.chat_list.as_ref()
    }

    pub(crate) fn set_chat_list(&mut self, chat_list: Option<ChatList>) {
        self.chat_list = chat_list;
    }

    pub(crate) fn chat_list_mut(&mut self) -> &mut Option<ChatList> {
        &mut self.chat_list
    }
}
