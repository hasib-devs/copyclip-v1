import { Smile } from "lucide-react";
import EmojiPicker from "emoji-picker-react";

const EmojiView = () => {
  return (
    <>
      <div className="p-4 border-b border-zinc-200 bg-white">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 rounded-md bg-linear-to-br from-amber-400 to-amber-600 flex items-center justify-center">
            <Smile className="h-4 w-4 text-white" />
          </div>
          <h1 className="text-xl font-semibold text-zinc-800">Emoji Picker</h1>
        </div>
        <p className="text-sm text-zinc-500 mt-1">
          Click an emoji to copy to clipboard
        </p>
      </div>

      <div className="flex-1 overflow-hidden">
        <EmojiPicker
          autoFocusSearch={false}
          width={"100%"}
          height={468}
          style={{ borderRadius: "0px" }}
        />
      </div>
    </>
  );
};

export default EmojiView;
