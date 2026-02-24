import EmojiPicker from "emoji-picker-react";

const EmojiView = () => {
  return (
    <>
      <div className="">
        <EmojiPicker
          autoFocusSearch={false}
          width={"100%"}
          height={665}
          style={{ borderRadius: "0px" }}
        />
      </div>
    </>
  );
};

export default EmojiView;
