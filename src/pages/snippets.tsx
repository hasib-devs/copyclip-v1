import { useState } from "react";
import { Plus, Copy, Trash2, Edit2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

const Snippets = () => {
  const [searchQuery, setSearchQuery] = useState("");
  const [snippets] = useState([
    {
      id: 1,
      title: "Email Signature",
      category: "email",
      content: "Best regards,\nYour Name\nposition@company.com",
    },
    {
      id: 2,
      title: "Meeting Template",
      category: "meetings",
      content:
        "Meeting Notes\n\nDate: \nAttendees: \n\nAgenda:\n\nAction Items:",
    },
    {
      id: 3,
      title: "Code Comment",
      category: "code",
      content: "// TODO: Fix this implementation",
    },
  ]);

  const filteredSnippets = snippets.filter(
    (s) =>
      s.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      s.content.toLowerCase().includes(searchQuery.toLowerCase()),
  );

  const categoryColors: Record<string, string> = {
    email: "bg-blue-100 text-blue-700",
    meetings: "bg-purple-100 text-purple-700",
    code: "bg-green-100 text-green-700",
    general: "bg-slate-100 text-slate-700",
  };

  return (
    <div className="flex flex-col h-full bg-white">
      {/* Header */}
      <div className="border-b border-slate-200 p-4">
        <div className="flex items-center gap-3 mb-4">
          <div className="w-8 h-8 rounded-lg bg-orange-100 flex items-center justify-center text-lg">
            ✂️
          </div>
          <h1 className="text-2xl font-bold text-slate-900">Snippets</h1>
        </div>
        <p className="text-sm text-slate-500 mb-3">
          Create and manage reusable text snippets
        </p>

        {/* Search Bar */}
        <div className="flex gap-2">
          <Input
            placeholder="Search snippets..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="bg-slate-50 border-slate-200"
          />
          <Button
            size="sm"
            className="bg-blue-600 hover:bg-blue-700 text-white"
          >
            <Plus size={16} className="mr-1" /> New
          </Button>
        </div>
      </div>

      {/* Snippets List */}
      <div className="flex-1 overflow-y-auto">
        {filteredSnippets.length === 0 ? (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <div className="text-4xl mb-3">✂️</div>
              <p className="text-slate-500 text-sm">No snippets found</p>
              <p className="text-slate-400 text-xs mt-1">
                Create your first snippet to get started
              </p>
            </div>
          </div>
        ) : (
          <div className="divide-y divide-slate-200">
            {filteredSnippets.map((snippet) => (
              <div
                key={snippet.id}
                className="p-4 hover:bg-slate-50 transition-colors"
              >
                <div className="flex items-start justify-between mb-2">
                  <div className="flex-1">
                    <h3 className="font-medium text-slate-900">
                      {snippet.title}
                    </h3>
                    <span
                      className={cn(
                        "inline-block text-xs font-medium px-2 py-1 rounded mt-1",
                        categoryColors[snippet.category] ||
                          categoryColors.general,
                      )}
                    >
                      {snippet.category}
                    </span>
                  </div>
                  <div className="flex gap-1">
                    <button
                      className="p-1.5 text-slate-400 hover:text-slate-600 hover:bg-slate-100 rounded"
                      title="Copy"
                    >
                      <Copy size={16} />
                    </button>
                    <button
                      className="p-1.5 text-slate-400 hover:text-slate-600 hover:bg-slate-100 rounded"
                      title="Edit"
                    >
                      <Edit2 size={16} />
                    </button>
                    <button
                      className="p-1.5 text-slate-400 hover:text-red-600 hover:bg-red-50 rounded"
                      title="Delete"
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                </div>
                <p className="text-sm text-slate-600 bg-slate-50 p-2 rounded font-mono whitespace-pre-wrap max-h-20 overflow-hidden">
                  {snippet.content}
                </p>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default Snippets;
