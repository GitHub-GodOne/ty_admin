import React from 'react';

interface SvgIconProps {
  name: string;
  size?: number;
  color?: string;
  className?: string;
}

const SvgIcon: React.FC<SvgIconProps> = ({ name, size = 16, color, className }) => {
  return (
    <svg
      className={className}
      style={{ width: size, height: size, fill: color || 'currentColor', verticalAlign: 'middle' }}
      aria-hidden="true"
    >
      <use xlinkHref={`#icon-${name}`} />
    </svg>
  );
};

export default SvgIcon;
