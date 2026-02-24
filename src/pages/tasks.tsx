import { useState } from "react";
import { Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { cn } from "@/lib/utils";

const Tasks = () => {
  const [filter, setFilter] = useState("all");

  const tasks = [
    {
      id: 1,
      title: "Complete project proposal",
      status: "pending",
      priority: "high",
    },
    {
      id: 2,
      title: "Schedule dentist appointment",
      status: "pending",
      priority: "low",
    },
    { id: 3, title: "Buy groceries", status: "completed", priority: "medium" },
  ];

  return (
    <div className="flex flex-col h-full bg-white">
      {/* Header */}
      <div className="border-b border-slate-200 p-4">
        <div className="flex items-center gap-3 mb-4">
          <div className="w-8 h-8 rounded-lg bg-green-100 flex items-center justify-center text-lg">
            ✓
          </div>
          <h1 className="text-2xl font-bold text-slate-900">Quick Tasks</h1>
        </div>
        <p className="text-sm text-slate-500">Manage your daily tasks</p>
      </div>

      {/* Filter Tabs */}
      <div className="flex gap-1 px-4 pt-3 border-b border-slate-200">
        {["All", "Active", "Completed"].map((f) => (
          <button
            key={f}
            onClick={() => setFilter(f.toLowerCase())}
            className={cn(
              "px-3 py-1.5 rounded text-sm font-medium transition-colors",
              filter === f.toLowerCase()
                ? "bg-slate-900 text-white"
                : "bg-slate-100 text-slate-700 hover:bg-slate-200",
            )}
          >
            {f}
          </button>
        ))}
        <div className="flex-1" />
        <Button
          size="sm"
          variant="outline"
          className="text-blue-600 border-blue-600"
        >
          <Plus size={16} className="mr-1" /> New
        </Button>
      </div>

      {/* Tasks List */}
      <div className="flex-1 overflow-y-auto">
        <div className="divide-y divide-slate-200">
          {tasks.map((task) => (
            <div
              key={task.id}
              className="flex items-center gap-3 p-4 hover:bg-slate-50 transition-colors"
            >
              <Checkbox
                className="w-5 h-5"
                checked={task.status === "completed"}
              />
              <div className="flex-1">
                <p
                  className={cn(
                    "font-medium",
                    task.status === "completed"
                      ? "line-through text-slate-400"
                      : "text-slate-900",
                  )}
                >
                  {task.title}
                </p>
                <div className="flex gap-3 mt-1 text-xs text-slate-500">
                  <span>📅 Due date</span>
                  <span
                    className={cn(
                      "px-2 py-0.5 rounded",
                      task.priority === "high"
                        ? "bg-red-100 text-red-700"
                        : task.priority === "medium"
                          ? "bg-yellow-100 text-yellow-700"
                          : "bg-blue-100 text-blue-700",
                    )}
                  >
                    {task.priority.charAt(0).toUpperCase() +
                      task.priority.slice(1)}
                  </span>
                </div>
              </div>
              <button className="text-slate-400 hover:text-slate-600">⋯</button>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default Tasks;
