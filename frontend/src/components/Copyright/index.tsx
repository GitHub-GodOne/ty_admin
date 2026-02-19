import React from 'react';

const Copyright: React.FC = () => {
  return (
    <div style={{ textAlign: 'center', padding: '16px 0', color: '#999', fontSize: 12 }}>
      Copyright &copy; {new Date().getFullYear()} TyAdmin. All rights reserved.
    </div>
  );
};

export default Copyright;
