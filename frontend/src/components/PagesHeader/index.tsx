import React from 'react';

interface PagesHeaderProps {
  title: string;
  desc?: string;
}

const PagesHeader: React.FC<PagesHeaderProps> = ({ title, desc }) => {
  return (
    <div style={{ marginBottom: 16 }}>
      <h3 style={{ fontSize: 16, fontWeight: 600, margin: 0 }}>{title}</h3>
      {desc && <p style={{ color: '#999', fontSize: 13, margin: '4px 0 0' }}>{desc}</p>}
    </div>
  );
};

export default PagesHeader;
