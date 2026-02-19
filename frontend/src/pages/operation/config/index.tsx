import React, { useState, useEffect, useCallback } from 'react';
import { Card, Tabs, Spin, message, Empty } from 'antd';
import { treeCategoryApi } from '@/api/category';
import { getFormConfigInfo } from '@/api/systemFormConfig';
import {
  systemConfigInfo,
  systemConfigSave,
} from '@/api/systemSetting';
import FormParser from '@/components/FormParser';

interface CategoryNode {
  id: number;
  name: string;
  extra?: string;
  child?: CategoryNode[];
}

interface FormConf {
  fields: any[];
  labelWidth?: number;
}

const OperationConfig: React.FC = () => {
  const [categoryTree, setCategoryTree] = useState<CategoryNode[]>([]);
  const [treeLoading, setTreeLoading] = useState(false);
  const [formConf, setFormConf] = useState<FormConf | null>(null);
  const [formData, setFormData] = useState<Record<string, any>>({});
  const [formLoading, setFormLoading] = useState(false);
  const [submitLoading, setSubmitLoading] = useState(false);
  const [currentFormId, setCurrentFormId] = useState<string>('');

  useEffect(() => {
    setTreeLoading(true);
    treeCategoryApi({ type: 6, status: 1 })
      .then((res: any) => {
        const tree = Array.isArray(res) ? res : [];
        setCategoryTree(tree);
        // Auto-select first child tab
        if (tree.length > 0 && tree[0].child?.length) {
          const firstChild = tree[0].child[0];
          if (firstChild.extra) {
            loadForm(firstChild.extra);
          }
        }
      })
      .catch(() => message.error('获取配置分类失败'))
      .finally(() => setTreeLoading(false));
  }, []);

  const loadForm = useCallback(async (formId: string) => {
    if (!formId) return;
    setCurrentFormId(formId);
    setFormLoading(true);
    setFormConf(null);
    setFormData({});
    try {
      // 两个请求独立处理，互不影响
      const [tempResult, configResult] = await Promise.allSettled([
        getFormConfigInfo({ id: formId }),
        systemConfigInfo({ id: formId }),
      ]);
      // 解析表单模板
      if (tempResult.status === 'fulfilled' && tempResult.value?.content) {
        const content = tempResult.value.content;
        const parsed = typeof content === 'string' ? JSON.parse(content) : content;
        setFormConf(parsed);
      } else if (tempResult.status === 'rejected') {
        console.error('获取表单模板失败:', tempResult.reason);
      }
      // 解析配置值
      if (configResult.status === 'fulfilled' && configResult.value && typeof configResult.value === 'object') {
        setFormData(configResult.value);
      } else if (configResult.status === 'rejected') {
        console.error('获取配置值失败:', configResult.reason);
      }
    } catch (e) {
      console.error('加载表单异常:', e);
      message.error('获取表单配置失败');
    } finally {
      setFormLoading(false);
    }
  }, []);

  const handleSubmit = async (values: Record<string, any>) => {
    if (!currentFormId || !formConf) return;
    setSubmitLoading(true);
    try {
      const fields = formConf.fields.map((field: any) => ({
        name: field.__vModel__,
        title: field.__config__?.label || '',
        value: values[field.__vModel__] ?? '',
      }));
      await systemConfigSave({ id: Number(currentFormId), fields });
      message.success('保存成功');
    } catch {
      /* handled by interceptor */
    } finally {
      setSubmitLoading(false);
    }
  };

  const handleChildTabChange = (key: string) => {
    loadForm(key);
  };

  if (treeLoading) {
    return (
      <Card>
        <Spin spinning style={{ width: '100%', padding: 80 }} />
      </Card>
    );
  }

  if (!categoryTree.length) {
    return (
      <Card>
        <Empty description="暂无配置分类" />
      </Card>
    );
  }

  return (
    <Card>
      <Tabs
        onChange={(key) => {
          const parent = categoryTree.find((c) => String(c.id) === key);
          if (parent?.child?.length) {
            const firstChild = parent.child[0];
            if (firstChild.extra) {
              loadForm(firstChild.extra);
            }
          }
        }}
        items={categoryTree.map((parent) => ({
          key: String(parent.id),
          label: parent.name,
          children: (
            <Tabs
              tabPosition="left"
              activeKey={currentFormId}
              onChange={handleChildTabChange}
              items={
                parent.child?.map((child) => ({
                  key: child.extra || String(child.id),
                  label: child.name,
                  children: (
                    <Spin spinning={formLoading}>
                      {formConf && currentFormId === (child.extra || String(child.id)) ? (
                        <FormParser
                          formConf={formConf}
                          formData={formData}
                          onSubmit={handleSubmit}
                          loading={submitLoading}
                        />
                      ) : !formLoading ? (
                        <Empty description="暂无表单配置" />
                      ) : null}
                    </Spin>
                  ),
                })) || []
              }
            />
          ),
        }))}
      />
    </Card>
  );
};

export default OperationConfig;
