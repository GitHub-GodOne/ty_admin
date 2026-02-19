import React, { useRef, useEffect } from 'react';
import * as echarts from 'echarts';

interface EChartsProps {
  option: echarts.EChartsOption;
  style?: React.CSSProperties;
  className?: string;
  loading?: boolean;
}

const EChartsComponent: React.FC<EChartsProps> = ({ option, style, className, loading }) => {
  const chartRef = useRef<HTMLDivElement>(null);
  const instanceRef = useRef<echarts.ECharts>();

  useEffect(() => {
    if (!chartRef.current) return;
    instanceRef.current = echarts.init(chartRef.current);
    const handleResize = () => instanceRef.current?.resize();
    window.addEventListener('resize', handleResize);
    return () => {
      window.removeEventListener('resize', handleResize);
      instanceRef.current?.dispose();
    };
  }, []);

  useEffect(() => {
    if (!instanceRef.current) return;
    if (loading) {
      instanceRef.current.showLoading();
    } else {
      instanceRef.current.hideLoading();
      instanceRef.current.setOption(option, true);
    }
  }, [option, loading]);

  return <div ref={chartRef} className={className} style={{ width: '100%', height: 300, ...style }} />;
};

export default EChartsComponent;
