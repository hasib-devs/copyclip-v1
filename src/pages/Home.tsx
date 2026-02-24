import { useState } from "react";
import { Search, Filter, Menu } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

const Home = () => {
  const [searchQuery, setSearchQuery] = useState("");

  return (
    <div className="flex flex-col h-full bg-white">
      {/* Header Section */}
      <div className="border-b border-slate-200 p-4">
        <div className="flex items-center gap-3 mb-4">
          <div className="w-8 h-8 rounded-lg bg-blue-100 flex items-center justify-center text-lg">
            📋
          </div>
          <h1 className="text-2xl font-bold text-slate-900">
            Clipboard History
          </h1>
        </div>

        {/* Search Bar */}
        <div className="flex gap-2">
          <div className="flex-1 relative">
            <Search
              className="absolute left-3 top-2.5 text-slate-400"
              size={18}
            />
            <Input
              placeholder="Search..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className={cn(
                "pl-10 bg-slate-50 border-slate-200",
                "focus:bg-white focus:border-blue-300",
              )}
            />
          </div>
          <Button variant="outline" size="sm" className="text-slate-600">
            <Filter size={18} />
          </Button>
          <Button variant="outline" size="sm" className="text-slate-600">
            <Menu size={18} />
          </Button>
        </div>
      </div>

      {/* Content Area */}
      <div className="flex-1 overflow-y-auto bg-white">
        <div className="flex items-center justify-center h-full">
          <div className="text-center">
            <div className="w-16 h-16 mx-auto mb-4 flex items-center justify-center text-4xl">
              📋
            </div>
            <p className="text-slate-500 text-sm">No clipboard items found</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Home;
