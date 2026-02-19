import React, { useRef, useEffect } from 'react';
import { Spin } from 'antd';

interface TinymceProps {
  value?: string;
  onChange?: (val: string) => void;
  height?: number;
  disabled?: boolean;
}

const Tinymce: React.FC<TinymceProps> = ({ value = '', onChange, height = 400, disabled = false }) => {
  const editorRef = useRef<any>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [loading, setLoading] = React.useState(true);

  useEffect(() => {
    const id = `tinymce-${Date.now()}`;
    if (!containerRef.current) return;

    const textarea = document.createElement('textarea');
    textarea.id = id;
    containerRef.current.appendChild(textarea);

    const initTinymce = () => {
      if (!(window as any).tinymce) {
        setLoading(false);
        return;
      }
      (window as any).tinymce.init({
        selector: `#${id}`,
        height,
        menubar: false,
        readonly: disabled,
        plugins: 'lists link image table code wordcount fullscreen',
        toolbar: 'undo redo | formatselect | bold italic | alignleft aligncenter alignright | bullist numlist | link image | code fullscreen',
        setup: (editor: any) => {
          editorRef.current = editor;
          editor.on('init', () => {
            editor.setContent(value || '');
            setLoading(false);
          });
          editor.on('change keyup', () => {
            onChange?.(editor.getContent());
          });
        },
      });
    };

    if ((window as any).tinymce) {
      initTinymce();
    } else {
      // Fallback: use a simple textarea
      setLoading(false);
    }

    return () => {
      if (editorRef.current) {
        editorRef.current.remove();
      }
    };
  }, []);

  // If tinymce is not loaded, show a simple textarea fallback
  if (!(window as any).tinymce) {
    return (
      <textarea
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
        style={{ width: '100%', height, border: '1px solid #d9d9d9', borderRadius: 6, padding: 8 }}
        disabled={disabled}
        placeholder="请输入内容..."
      />
    );
  }

  return (
    <Spin spinning={loading}>
      <div ref={containerRef} />
    </Spin>
  );
};

export default Tinymce;
