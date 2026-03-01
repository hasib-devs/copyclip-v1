import { cn } from "@/lib/utils";
import { BarChart3, Code2, Settings, Smile } from "lucide-react";
import { Link, useLocation } from "react-router";

const Navigation = () => {
  const location = useLocation();

  const navItems = [
    { to: "/emoji", icon: Smile, label: "Emoji" },
    { to: "/snippets", icon: Code2, label: "Snippets" },
    { to: "/stats", icon: BarChart3, label: "Stats" },
    { to: "/settings", icon: Settings, label: "Settings" },
  ];

  return (
    <nav
      className={cn(
        "flex items-center justify-between gap-4 px-4",
        "bg-white border-b border-slate-200",
        "overflow-x-auto",
      )}
    >
      <div className="flex items-center justify-around gap-0 flex-1">
        {navItems.map(({ to, icon: Icon, label }) => {
          const isActive = location.pathname === to;

          return (
            <Link
              key={to}
              to={to}
              className={cn(
                "flex flex-col items-center justify-center gap-1",
                "min-w-fit px-1 pt-3 pb-1",
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
      </div>
    </nav>
  );
};

export default Navigation;
