import { Modal } from 'antd';

export function useModal() {
  const confirm = (title: string, content: string, onOk: () => void | Promise<void>) => {
    Modal.confirm({
      title,
      content,
      okText: '确定',
      cancelText: '取消',
      onOk,
    });
  };

  const deleteConfirm = (onOk: () => void | Promise<void>) => {
    confirm('确认删除', '确定要删除吗？此操作不可恢复。', onOk);
  };

  return { confirm, deleteConfirm };
}
