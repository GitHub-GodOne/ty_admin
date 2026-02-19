import React, { useState } from 'react';
import { DatePicker, Space, Button } from 'antd';
import dayjs, { Dayjs } from 'dayjs';

const { RangePicker } = DatePicker;

interface TimeSelectProps {
  onChange?: (dates: { start: string; end: string } | null) => void;
  showCustom?: boolean;
}

const presets: { label: string; key: string; range: () => [Dayjs, Dayjs] }[] = [
  { label: '今天', key: 'today', range: () => [dayjs().startOf('day'), dayjs().endOf('day')] },
  { label: '昨天', key: 'yesterday', range: () => [dayjs().subtract(1, 'day').startOf('day'), dayjs().subtract(1, 'day').endOf('day')] },
  { label: '最近7天', key: '7days', range: () => [dayjs().subtract(6, 'day').startOf('day'), dayjs().endOf('day')] },
  { label: '最近30天', key: '30days', range: () => [dayjs().subtract(29, 'day').startOf('day'), dayjs().endOf('day')] },
  { label: '本月', key: 'month', range: () => [dayjs().startOf('month'), dayjs().endOf('day')] },
  { label: '本年', key: 'year', range: () => [dayjs().startOf('year'), dayjs().endOf('day')] },
];

const TimeSelect: React.FC<TimeSelectProps> = ({ onChange, showCustom = true }) => {
  const [active, setActive] = useState<string>('');

  const handlePreset = (key: string, range: () => [Dayjs, Dayjs]) => {
    setActive(key);
    const [start, end] = range();
    onChange?.({ start: start.format('YYYY-MM-DD'), end: end.format('YYYY-MM-DD') });
  };

  const handleRange = (_: any, dateStrings: [string, string]) => {
    setActive('custom');
    if (dateStrings[0] && dateStrings[1]) {
      onChange?.({ start: dateStrings[0], end: dateStrings[1] });
    } else {
      onChange?.(null);
    }
  };

  return (
    <Space wrap>
      {presets.map((p) => (
        <Button key={p.key} type={active === p.key ? 'primary' : 'default'} size="small" onClick={() => handlePreset(p.key, p.range)}>
          {p.label}
        </Button>
      ))}
      {showCustom && <RangePicker size="small" onChange={handleRange} />}
    </Space>
  );
};

export default TimeSelect;
