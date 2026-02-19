import React, { useRef, useState } from 'react';
import { Input, AutoComplete } from 'antd';
import { SearchOutlined } from '@ant-design/icons';

interface HeaderSearchProps {
  options?: { label: string; value: string; path: string }[];
  onSelect?: (path: string) => void;
}

const HeaderSearch: React.FC<HeaderSearchProps> = ({ options = [], onSelect }) => {
  const [visible, setVisible] = useState(false);
  const [searchVal, setSearchVal] = useState('');

  const filtered = searchVal
    ? options.filter((o) => o.label.toLowerCase().includes(searchVal.toLowerCase()))
    : [];

  return visible ? (
    <AutoComplete
      style={{ width: 200 }}
      options={filtered.map((o) => ({ value: o.path, label: o.label }))}
      onSelect={(val) => { onSelect?.(val); setVisible(false); setSearchVal(''); }}
      onBlur={() => setVisible(false)}
      value={searchVal}
      onChange={setSearchVal}
    >
      <Input size="small" placeholder="搜索菜单" autoFocus prefix={<SearchOutlined />} />
    </AutoComplete>
  ) : (
    <SearchOutlined style={{ cursor: 'pointer', fontSize: 16 }} onClick={() => setVisible(true)} />
  );
};

export default HeaderSearch;
