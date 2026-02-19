import { useState, useCallback, useEffect } from 'react';
import { message } from 'antd';
import { usePagination } from './usePagination';

interface UseTableListOptions<T, P> {
  apiFn: (params: P & { page: number; limit: number }) => Promise<any>;
  searchParams?: P;
  defaultLimit?: number;
  autoLoad?: boolean;
}

export function useTableList<T = any, P = Record<string, any>>(options: UseTableListOptions<T, P>) {
  const { apiFn, searchParams, defaultLimit = 20, autoLoad = true } = options;
  const { pagination, setTotal, antdPagination } = usePagination(defaultLimit);
  const [list, setList] = useState<T[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchList = useCallback(async (page?: number) => {
    setLoading(true);
    try {
      const params = {
        ...searchParams,
        page: page || pagination.page,
        limit: pagination.limit,
      } as P & { page: number; limit: number };
      const res = await apiFn(params);
      if (res?.list) {
        setList(res.list);
        setTotal(res.total || res.count || 0);
      } else if (Array.isArray(res)) {
        setList(res);
        setTotal(res.length);
      }
    } catch (err: any) {
      message.error(err?.message || '获取列表失败');
    } finally {
      setLoading(false);
    }
  }, [apiFn, searchParams, pagination.page, pagination.limit, setTotal]);

  useEffect(() => {
    if (autoLoad) fetchList();
  }, [pagination.page, pagination.limit]);

  return { list, loading, fetchList, antdPagination };
}
