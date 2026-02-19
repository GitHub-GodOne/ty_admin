import React, { useState, useEffect, useCallback } from 'react';
import { Card, Table, Tag, message } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import { jobLogList } from '@/api/schedule';

const ScheduleLogList: React.FC = () => {
  const [list, setList] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 20, total: 0 });

  const fetchList = useCallback(async (page = 1) => {
    setLoading(true);
    try {
      const res = await jobLogList({ page, limit: pagination.pageSize });
      setList(res?.list || []);
      setPagination((p) => ({ ...p, current: page, total: res?.total || 0 }));
    } catch { message.error('获取任务日志失败'); }
    finally { setLoading(false); }
  }, [pagination.pageSize]);

  useEffect(() => { fetchList(1); }, []);

  const columns: ColumnsType<any> = [
    { title: '任务ID', dataIndex: 'jobId', width: 80 },
    { title: '日志ID', dataIndex: 'logId', width: 80 },
    { title: 'Bean名称', dataIndex: 'beanName', width: 150 },
    { title: '方法名称', dataIndex: 'methodName', width: 150 },
    { title: '参数', dataIndex: 'params', ellipsis: true, width: 150 },
    { title: '耗时(ms)', dataIndex: 'times', width: 100 },
    { title: '异常信息', dataIndex: 'error', ellipsis: true, width: 200 },
    { title: '创建时间', dataIndex: 'createTime', width: 170 },
  ];

  return (
    <Card>
      <Table rowKey="logId" columns={columns} dataSource={list} loading={loading}
        size="small" scroll={{ x: 1100 }}
        pagination={{
          ...pagination, showSizeChanger: true, pageSizeOptions: ['20', '40', '60', '80'],
          showTotal: (t: number) => `共 ${t} 条`,
          onChange: (p, ps) => { setPagination((prev) => ({ ...prev, pageSize: ps || 20 })); fetchList(p); },
        }} />
    </Card>
  );
};

export default ScheduleLogList;
