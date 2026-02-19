import React, { useState } from 'react';
import { Upload, Modal, message, Button } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import type { UploadFile, UploadProps } from 'antd';
import { getToken } from '@/utils/auth';

interface UploadPictureProps {
  value?: string | string[];
  onChange?: (val: string | string[]) => void;
  limit?: number;
  multiple?: boolean;
  action?: string;
}

const UploadPicture: React.FC<UploadPictureProps> = ({
  value,
  onChange,
  limit = 1,
  multiple = false,
  action = '/api/admin/upload/image',
}) => {
  const [previewOpen, setPreviewOpen] = useState(false);
  const [previewImage, setPreviewImage] = useState('');

  const fileList: UploadFile[] = (() => {
    if (!value) return [];
    const urls = Array.isArray(value) ? value : [value];
    return urls.filter(Boolean).map((url, i) => ({
      uid: `${i}`,
      name: `image-${i}`,
      status: 'done' as const,
      url,
    }));
  })();

  const handlePreview = (file: UploadFile) => {
    setPreviewImage(file.url || file.thumbUrl || '');
    setPreviewOpen(true);
  };

  const handleChange: UploadProps['onChange'] = ({ fileList: newFileList }) => {
    const urls = newFileList
      .filter((f) => f.status === 'done')
      .map((f) => f.response?.data?.url || f.url)
      .filter(Boolean);
    if (multiple) {
      onChange?.(urls);
    } else {
      onChange?.(urls[0] || '');
    }
  };

  const handleRemove = () => {
    onChange?.(multiple ? [] : '');
  };

  return (
    <>
      <Upload
        listType="picture-card"
        fileList={fileList}
        onPreview={handlePreview}
        onChange={handleChange}
        onRemove={handleRemove}
        multiple={multiple}
        maxCount={limit}
        headers={{ 'Authori-zation': getToken() || '' }}
        action={action}
        accept="image/*"
      >
        {fileList.length < limit && (
          <div>
            <PlusOutlined />
            <div style={{ marginTop: 8 }}>上传</div>
          </div>
        )}
      </Upload>
      <Modal open={previewOpen} footer={null} onCancel={() => setPreviewOpen(false)}>
        <img style={{ width: '100%' }} src={previewImage} alt="preview" />
      </Modal>
    </>
  );
};

export default UploadPicture;
