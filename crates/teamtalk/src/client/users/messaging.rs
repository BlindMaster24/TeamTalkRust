use super::*;

impl Client {
    /// Sends a text message to a target.
    pub fn send_text<T: Into<MessageTarget>>(&self, target: T, text: &str) -> i32 {
        self.send_text_with_options(target, text, SendTextOptions::default())
    }

    /// Sends a text message to a target using explicit options.
    pub fn send_text_with_options<T: Into<MessageTarget>>(
        &self,
        target: T,
        text: &str,
        options: SendTextOptions,
    ) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        let target = target.into();
        let chunks = split_text_chunks(text, TT_TEXT_MAX_PAYLOAD);
        let total_chunks = chunks.len();
        let mut last_cmd_id = 0;

        for (index, chunk) in chunks.into_iter().enumerate() {
            let mut msg = text_message_for_target(target);
            msg.bMore = if index + 1 < total_chunks { 1 } else { 0 };
            let tt = chunk.tt();
            unsafe {
                let len = tt.len().min(TT_TEXT_MAX_PAYLOAD);
                std::ptr::copy_nonoverlapping(tt.as_ptr(), msg.szMessage.as_mut_ptr(), len);
            }

            let mut retries_left = if index == 0 {
                options.first_chunk_retries
            } else {
                0
            };
            loop {
                let cmd_id = self.backend().do_text_message(self.ptr.0, &msg);
                if cmd_id > 0 {
                    last_cmd_id = cmd_id;
                    break;
                }
                if retries_left == 0 {
                    return cmd_id;
                }
                retries_left -= 1;
            }
        }

        last_cmd_id
    }

    /// Sends a text message to a user.
    pub fn send_to_user(&self, user_id: UserId, text: &str) -> i32 {
        self.send_text(MessageTarget::User(user_id), text)
    }

    /// Sends a text message to a channel.
    pub fn send_to_channel(&self, channel_id: ChannelId, text: &str) -> i32 {
        self.send_text(MessageTarget::Channel(channel_id), text)
    }

    /// Sends a text message to all users.
    pub fn send_to_all(&self, text: &str) -> i32 {
        self.send_text(MessageTarget::Broadcast, text)
    }
}
