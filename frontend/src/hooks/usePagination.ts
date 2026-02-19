import { useState, useCallback } from 'react';

interface PaginationState {
  page: number;
  limit: number;
  total: number;
}

export function usePagination(defaultLimit = 20) {
  const [pagination, setPagination] = useState<PaginationState>({
    page: 1,
    limit: defaultLimit,
    total: 0,
  });

  const setPage = useCallback((page: number) => {
    setPagination((prev) => ({ ...prev, page }));
  }, []);

  const setLimit = useCallback((limit: number) => {
    setPagination((prev) => ({ ...prev, limit, page: 1 }));
  }, []);

  const setTotal = useCallback((total: number) => {
    setPagination((prev) => ({ ...prev, total }));
  }, []);

  const antdPagination = {
    current: pagination.page,
    pageSize: pagination.limit,
    total: pagination.total,
    showSizeChanger: true,
    showQuickJumper: true,
    showTotal: (total: number) => `共 ${total} 条`,
    onChange: (page: number, pageSize: number) => {
      setPagination({ page, limit: pageSize, total: pagination.total });
    },
  };

  return { pagination, setPage, setLimit, setTotal, antdPagination };
}
