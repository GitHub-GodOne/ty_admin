import React, { useState, useEffect, useCallback } from 'react';
import {
  Modal, Button, Upload, Space, message,
  Empty, Spin, Typography,
} from 'antd';
import { PlusOutlined, CheckCircleFilled } from '@ant-design/icons';
import { getToken } from '@/utils/auth';
import { fileListApi } from '@/api/systemSetting';

interface MaterialPickerProps {
  open: boolean;
  onCancel: () => void;
  onOk: (urls: string[]) => void;
  multiple?: boolean;
  limit?: number;
  accept?: string;
}

// __CONTINUE_HERE__

const MaterialPicker: React.FC<MaterialPickerProps> = ({
  open, onCancel, onOk, multiple = false, limit = 1, accept = 'image',
}) => {
  const [fileList, setFileList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedUrls, setSelectedUrls] = useState<string[]>([]);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  const fetchFiles = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await fileListApi({ page, limit: pagination.pageSize });
      setFileList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { /* noop */ }
    finally { setLoading(false); }
  }, [pagination.pageSize]);

  useEffect(() => { if (open) { fetchFiles(1); setSelectedUrls([]); } }, [open]);

  const getUrl = (file: any) => file.attDir || file.sattDir || file.imageUrl || '';

  const handleSelect = (url: string) => {
    if (!multiple) { setSelectedUrls([url]); return; }
    setSelectedUrls((prev) => {
      if (prev.includes(url)) return prev.filter((u) => u !== url);
      if (prev.length >= limit) { message.warning(`最多选择${limit}个`); return prev; }
      return [...prev, url];
    });
  };

  const handleConfirm = () => {
    if (selectedUrls.length === 0) { message.warning('请选择素材'); return; }
    onOk(selectedUrls);
    setSelectedUrls([]);
  };

  const uploadProps: any = {
    name: 'file',
    action: '/api/admin/upload/image',
    headers: { 'Authori-zation': getToken() || '' },
    showUploadList: false,
    accept: accept === 'video' ? 'video/*' : 'image/*',
    onChange(info: any) {
      if (info.file.status === 'done') { message.success('上传成功'); fetchFiles(pagination.current); }
      else if (info.file.status === 'error') { message.error('上传失败'); }
    },
  };

// __CONTINUE_RENDER__

  const totalPages = Math.ceil(pagination.total / pagination.pageSize);

  return (
    <Modal title="选择素材" open={open} onCancel={onCancel} onOk={handleConfirm}
      width={720} destroyOnClose styles={{ body: { padding: '16px' } }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 12 }}>
        <Typography.Text type="secondary">
          已选 {selectedUrls.length}{multiple ? ` / ${limit}` : ''} 个
        </Typography.Text>
        <Upload {...uploadProps}>
          <Button size="small" type="primary" icon={<PlusOutlined />}>上传</Button>
        </Upload>
      </div>
      <Spin spinning={loading}>
        {fileList.length === 0 ? (
          <Empty description="暂无素材" style={{ marginTop: 60 }} />
        ) : (
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(100px, 1fr))', gap: 10 }}>
            {fileList.map((file: any) => {
              const url = getUrl(file);
              const selected = selectedUrls.includes(url);
              return (
                <div key={file.attId || file.id} onClick={() => handleSelect(url)}
                  style={{ cursor: 'pointer', border: selected ? '2px solid #1677ff' : '2px solid transparent',
                    borderRadius: 6, overflow: 'hidden', position: 'relative', background: '#fafafa' }}>
                  <img src={url} alt="" style={{ width: '100%', height: 90, objectFit: 'cover', display: 'block' }} />
                  {selected && (
                    <CheckCircleFilled style={{ position: 'absolute', top: 4, right: 4, fontSize: 18, color: '#1677ff' }} />
                  )}
                  <div style={{ padding: '2px 4px', fontSize: 11, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                    {file.name || file.realName || '未命名'}
                  </div>
                </div>
              );
            })}
          </div>
        )}
        {totalPages > 1 && (
          <div style={{ textAlign: 'center', marginTop: 12 }}>
            <Button size="small" disabled={pagination.current <= 1}
              onClick={() => fetchFiles(pagination.current - 1)} style={{ marginRight: 8 }}>上一页</Button>
            <Typography.Text style={{ fontSize: 12 }}>
              {pagination.current} / {totalPages}
            </Typography.Text>
            <Button size="small" disabled={pagination.current >= totalPages}
              onClick={() => fetchFiles(pagination.current + 1)} style={{ marginLeft: 8 }}>下一页</Button>
          </div>
        )}
      </Spin>
    </Modal>
  );
};

export default MaterialPicker;
