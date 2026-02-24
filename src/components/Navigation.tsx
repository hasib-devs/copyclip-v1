import { Link, useLocation } from "react-router";
import {
  Clipboard,
  Smile,
  Code2,
  BarChart3,
  Settings,
  NotebookIcon,
} from "lucide-react";
import { cn } from "@/lib/utils";

const Navigation = () => {
  const location = useLocation();

  const navItems = [
    { to: "/", icon: Clipboard, label: "Clipboard" },
    { to: "/emoji", icon: Smile, label: "Emoji" },
    { to: "/tasks", icon: NotebookIcon, label: "Tasks" },
    { to: "/snippets", icon: Code2, label: "Snippets" },
    { to: "/stats", icon: BarChart3, label: "Stats" },
    { to: "/settings", icon: Settings, label: "Settings" },
  ];

  return (
    <nav
      className={cn(
        "flex items-center gap-0 px-4 py-2",
        "bg-white border-b border-slate-200",
        "overflow-x-auto",
      )}
    >
      {navItems.map(({ to, icon: Icon, label }) => {
        const isActive = location.pathname === to;

        return (
          <Link
            key={to}
            to={to}
            className={cn(
              "flex flex-col items-center justify-center gap-1",
              "px-4 py-3 min-w-fit",
              "text-xs font-medium transition-colors",
              "border-b-2",
              isActive
                ? "border-blue-600 text-slate-900"
                : "border-transparent text-slate-600 hover:text-slate-900 hover:bg-slate-50",
            )}
          >
            <Icon size={20} />
            <span>{label}</span>
          </Link>
        );
      })}
    </nav>
  );
};

export default Navigation;
