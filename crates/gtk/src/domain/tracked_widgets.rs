use crate::gui::{chat::ChatView, chats::ChatList};

#[derive(Debug, Default)]
pub(crate) struct TrackedWidgets {
    lbl_listener_status: Option<gtk::Label>,
    chat_list: Option<ChatList>,
    chat_view: Option<ChatView>,
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

    pub(crate) fn chat_list_mut(&mut self) -> Option<&mut ChatList> {
        self.chat_list.as_mut()
    }

    pub(crate) fn chat_view(&self) -> Option<&ChatView> {
        self.chat_view.as_ref()
    }

    pub(crate) fn chat_view_mut(&mut self) -> &mut Option<ChatView> {
        &mut self.chat_view
    }

    pub(crate) fn set_chat_view(&mut self, chat_view: Option<ChatView>) {
        self.chat_view = chat_view;
    }
}
