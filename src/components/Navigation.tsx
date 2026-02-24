import { Link, useLocation } from "react-router";
import { Clipboard, Smile, Code2, BarChart3, Settings } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";

const Navigation = () => {
  const location = useLocation();

  const navItems = [
    { to: "/", icon: Clipboard, label: "Home", title: "Clipboard History" },
    { to: "/emoji", icon: Smile, label: "Emoji", title: "Emoji & GIF" },
    { to: "/snippets", icon: Code2, label: "Snippets", title: "Snippets" },
    { to: "/stats", icon: BarChart3, label: "Stats", title: "Statistics" },
    { to: "/settings", icon: Settings, label: "Settings", title: "Settings" },
  ];

  return (
    <nav
      className={cn(
        "flex items-center justify-around gap-2",
        "px-3 py-2",
        "bg-slate-800 border-t border-slate-700",
      )}
    >
      {navItems.map(({ to, icon: Icon, title }) => {
        const isActive = location.pathname === to;

        return (
          <Tooltip key={to}>
            <TooltipTrigger asChild>
              <Link to={to}>
                <Button
                  variant={isActive ? "default" : "ghost"}
                  size="icon-sm"
                  className={cn(
                    "flex flex-col items-center justify-center",
                    isActive
                      ? "bg-blue-600 hover:bg-blue-700 text-white"
                      : "text-slate-400 hover:bg-slate-700 hover:text-slate-200",
                  )}
                >
                  <Icon size={18} />
                </Button>
              </Link>
            </TooltipTrigger>
            <TooltipContent side="top" className="text-xs">
              {title}
            </TooltipContent>
          </Tooltip>
        );
      })}
    </nav>
  );
};

export default Navigation;
