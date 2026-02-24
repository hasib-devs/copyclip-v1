import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { cn } from "@/lib/utils";

const Statistics = () => {
  return (
    <div className={cn("flex flex-col h-full", "p-4 gap-4")}>
      <Card className={cn("flex-1", "border-slate-700 bg-slate-800")}>
        <CardHeader>
          <CardTitle className="text-lg">Statistics</CardTitle>
          <CardDescription>
            View your clipboard usage insights and analytics
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div
            className={cn(
              "flex items-center justify-center h-full",
              "rounded-lg border-2 border-dashed border-slate-600",
              "text-slate-400 text-sm",
            )}
          >
            Analytics dashboard coming soon!
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default Statistics;
