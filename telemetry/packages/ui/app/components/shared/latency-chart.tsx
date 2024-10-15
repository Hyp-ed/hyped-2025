import { PreviousLatenciesType } from '@/context/pods';
import { LineChart } from '@tremor/react';

/**
 * Wrapper around LineChart to display latency data.
 * @param data The latency values to display, possibly undefined.
 * @param minValue The minimum value to display on the y-axis.
 * @param maxValue The maximum value to display on the y-axis.
 * @returns A LineChart component.
 */
export const LatencyChart = ({
  data,
  minValue = 0,
  maxValue = 100,
}: {
  data: PreviousLatenciesType | undefined;
  minValue?: number;
  maxValue?: number;
}) => {
  return (
    <div className="w-[200px]">
      {data && data.length > 0 ? (
        <LineChart
          className="h-[70px]"
          data={data}
          index="time"
          categories={['latency']}
          colors={['blue']}
          valueFormatter={dataFormatter}
          yAxisWidth={40}
          showYAxis={true}
          showXAxis={false}
          showLegend={false}
          showGridLines={true}
          maxValue={maxValue}
          minValue={minValue}
        />
      ) : (
        <p className="text-sm">No latencies to show</p>
      )}
    </div>
  );
};

const dataFormatter = (number: number) => `${number.toString()}ms`;
