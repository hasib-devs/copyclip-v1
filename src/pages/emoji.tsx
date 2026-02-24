import { useState } from "react";
import { Search } from "lucide-react";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

const Emoji = () => {
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedCategory, setSelectedCategory] = useState("recent");

  const categories = [
    { id: "recent", icon: "🕐", label: "Recent" },
    { id: "smileys", icon: "😀", label: "Smileys" },
    { id: "nature", icon: "🌲", label: "Nature" },
    { id: "food", icon: "🍕", label: "Food" },
  ];

  return (
    <div className="flex flex-col h-full bg-white">
      {/* Header */}
      <div className="border-b border-slate-200 p-4">
        <div className="flex items-center gap-3 mb-4">
          <div className="w-8 h-8 rounded-lg bg-yellow-100 flex items-center justify-center text-lg">
            😊
          </div>
          <h1 className="text-2xl font-bold text-slate-900">Emoji Picker</h1>
        </div>
        <p className="text-sm text-slate-500 mb-3">
          Click an emoji to copy to clipboard
        </p>
        <div className="relative">
          <Search
            className="absolute left-3 top-2.5 text-slate-400"
            size={18}
          />
          <Input
            placeholder="Search"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-10 bg-slate-50 border-slate-200"
          />
        </div>
      </div>

      {/* Category Tabs */}
      <div className="flex gap-1 px-4 pt-3 border-b border-slate-200 overflow-x-auto">
        {categories.map((cat) => (
          <button
            key={cat.id}
            onClick={() => setSelectedCategory(cat.id)}
            className={cn(
              "px-3 py-2 rounded-t-lg text-2xl transition-colors",
              selectedCategory === cat.id
                ? "bg-slate-100"
                : "hover:bg-slate-50",
            )}
            title={cat.label}
          >
            {cat.icon}
          </button>
        ))}
      </div>

      {/* Emoji Grid */}
      <div className="flex-1 overflow-y-auto p-4">
        <div className="grid grid-cols-6 gap-2">
          {Array.from({ length: 36 }).map((_, i) => (
            <button
              key={i}
              className="text-2xl p-2 rounded hover:bg-slate-100 transition-colors"
            >
              😊
            </button>
          ))}
        </div>
      </div>

      {/* Mood Section */}
      <div className="border-t border-slate-200 p-4">
        <div className="flex items-center gap-2 text-slate-600 text-sm">
          <span className="text-2xl">😊</span>
          <span>What's Your Mood?</span>
        </div>
      </div>
    </div>
  );
};

export default Emoji;
