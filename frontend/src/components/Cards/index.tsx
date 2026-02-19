import React from 'react';
import { Card, Statistic } from 'antd';

interface CardsProps {
  title: string;
  value: number | string;
  prefix?: React.ReactNode;
  suffix?: string;
  color?: string;
  loading?: boolean;
}

const Cards: React.FC<CardsProps> = ({ title, value, prefix, suffix, color, loading }) => {
  return (
    <Card hoverable bodyStyle={{ padding: '20px 24px' }} loading={loading}>
      <Statistic title={title} value={value} prefix={prefix} suffix={suffix} valueStyle={color ? { color } : undefined} />
    </Card>
  );
};

export default Cards;
