import EmojiPicker from "emoji-picker-react";

const EmojiView = () => {
  // Handle copy action
  const handleCopy = async (content: string) => {
    try {
      await navigator.clipboard.writeText(content);
    } catch (err) {
      console.error("Failed to copy to clipboard", err);
    }
  };

  return (
    <>
      <div className="">
        <EmojiPicker
          autoFocusSearch={false}
          width={"100%"}
          height={665}
          style={{ borderRadius: "0px" }}
          onEmojiClick={(emojiObject) => handleCopy(emojiObject.emoji)}
        />
      </div>
    </>
  );
};

export default EmojiView;
