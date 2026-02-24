import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

const Statistics = () => {
  return (
    <div className="flex flex-col h-full bg-white">
      {/* Header */}
      <div className="border-b border-slate-200 p-4">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg bg-purple-100 flex items-center justify-center text-lg">
            📊
          </div>
          <div>
            <h1 className="text-2xl font-bold text-slate-900">Statistics</h1>
            <p className="text-sm text-slate-500">
              View your clipboard usage insights
            </p>
          </div>
        </div>
      </div>

      {/* Stats Grid */}
      <div className="flex-1 overflow-y-auto p-6">
        <div className="grid grid-cols-2 gap-4 mb-6">
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium text-slate-600">
                Copies Today
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold text-slate-900">0</div>
              <p className="text-xs text-slate-500 mt-1">times</p>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium text-slate-600">
                Total Items
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold text-slate-900">0</div>
              <p className="text-xs text-slate-500 mt-1">stored</p>
            </CardContent>
          </Card>
        </div>
        <div className="text-center text-slate-500 text-sm">
          <p>More analytics coming soon!</p>
        </div>
      </div>
    </div>
  );
};

export default Statistics;
