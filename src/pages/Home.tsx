import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { cn } from "@/lib/utils";

const Home = () => {
  return (
    <div className={cn("flex flex-col h-full", "p-4 gap-4")}>
      <Card className={cn("flex-1", "border-slate-700 bg-slate-800")}>
        <CardHeader>
          <CardTitle className="text-lg">Clipboard History</CardTitle>
          <CardDescription>
            Your recent clipboard items will appear here
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
            No items yet. Start copying to see history!
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default Home;
