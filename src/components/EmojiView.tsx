import { useClipboard } from "@/hooks/useClipboard";
import { ItemType } from "@/types/clipboard";
import EmojiPicker from "emoji-picker-react";

const EmojiView = () => {
  // Get clipboard context
  const { copyToClipboard, setError } = useClipboard();

  // Handle copy action
  const handleCopy = async (
    content: string,
    type: ItemType["type"] = "text",
  ) => {
    try {
      await copyToClipboard(content, type);
    } catch (err) {
      setError("Failed to copy to clipboard");
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
