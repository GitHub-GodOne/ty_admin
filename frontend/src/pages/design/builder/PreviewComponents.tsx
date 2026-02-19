import React from 'react';

interface Props {
  comp: {
    name: string;
    cname: string;
    isHide: boolean;
    defaultConfig: Record<string, any>;
  };
}

const ComponentPreview: React.FC<Props> = ({ comp }) => {
  const cfg = comp.defaultConfig;

  switch (comp.name) {
    case 'banner':
      return (
        <div style={{ background: cfg.bgColor || '#fff', padding: '0' }}>
          <div style={{ height: 150, background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#fff', fontSize: 16 }}>
            轮播图 ({cfg.swiperConfig?.imgList?.length || 0} 张)
          </div>
          <div style={{ display: 'flex', justifyContent: 'center', gap: 4, padding: 6 }}>
            {(cfg.swiperConfig?.imgList || []).map((_: any, i: number) => (
              <div key={i} style={{ width: 6, height: 6, borderRadius: '50%', background: i === 0 ? '#1890ff' : '#ddd' }} />
            ))}
          </div>
        </div>
      );

    case 'home_title':
      return (
        <div style={{ background: cfg.bgColor || '#fff', padding: `${cfg.padding?.top || 10}px ${cfg.padding?.right || 12}px ${cfg.padding?.bottom || 10}px ${cfg.padding?.left || 12}px`, textAlign: cfg.textAlign || 'left' }}>
          <div style={{ fontSize: cfg.fontSize || 16, fontWeight: 600, color: cfg.titleColor || '#333' }}>{cfg.title || '标题文字'}</div>
          {cfg.subtitle && <div style={{ fontSize: 12, color: cfg.subtitleColor || '#999', marginTop: 2 }}>{cfg.subtitle}</div>}
        </div>
      );

    case 'home_menu':
      return (
        <div style={{ background: cfg.bgColor || '#fff', padding: `${cfg.padding?.top || 10}px ${cfg.padding?.right || 0}px ${cfg.padding?.bottom || 10}px ${cfg.padding?.left || 0}px` }}>
          <div style={{ display: 'grid', gridTemplateColumns: `repeat(${cfg.rowCount || 4}, 1fr)`, gap: 8 }}>
            {(cfg.menuList || []).map((item: any, i: number) => (
              <div key={i} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 4 }}>
                <div style={{ width: 44, height: 44, borderRadius: 8, background: '#f0f0f0', display: 'flex', alignItems: 'center', justifyContent: 'center', fontSize: 10, color: '#999' }}>图标</div>
                <span style={{ fontSize: 11, color: cfg.titleColor || '#333' }}>{item.title}</span>
              </div>
            ))}
          </div>
        </div>
      );
    case 'search_box':
      return (
        <div style={{ background: cfg.bgColor || '#fff', padding: '8px 12px' }}>
          <div style={{ background: '#f5f5f5', border: `1px solid ${cfg.borderColor || '#eee'}`, borderRadius: cfg.borderRadius || 20, padding: '6px 12px', color: '#bbb', fontSize: 13, textAlign: cfg.textAlign || 'left' }}>
            🔍 {cfg.placeholder || '搜索商品'}
          </div>
        </div>
      );

    case 'nav_bar':
      return (
        <div style={{ background: cfg.bgColor || '#fff', padding: '10px 12px', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
          {cfg.showBack && <span style={{ position: 'absolute', left: 12, fontSize: 16 }}>←</span>}
          <span style={{ fontWeight: 600, color: cfg.titleColor || '#333' }}>{cfg.title || '页面标题'}</span>
        </div>
      );

    case 'home_footer':
      return (
        <div style={{ background: cfg.bgColor || '#fff', display: 'flex', borderTop: '1px solid #f0f0f0', padding: '6px 0' }}>
          {(cfg.menuList || []).map((item: any, i: number) => (
            <div key={i} style={{ flex: 1, display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 2 }}>
              <div style={{ width: 22, height: 22, borderRadius: 4, background: '#f0f0f0' }} />
              <span style={{ fontSize: 10, color: i === 0 ? (cfg.activeColor || '#e93323') : (cfg.inactiveColor || '#999') }}>{item.title}</span>
            </div>
          ))}
        </div>
      );

    case 'home_comb':
      return (
        <div style={{ background: cfg.bgColor || '#e93323', padding: '10px 12px', display: 'flex', alignItems: 'center', gap: 8 }}>
          {cfg.showLogo && <div style={{ width: 30, height: 30, borderRadius: 6, background: 'rgba(255,255,255,0.3)' }} />}
          {cfg.showSearch && (
            <div style={{ flex: 1, background: 'rgba(255,255,255,0.3)', borderRadius: 16, padding: '4px 12px', color: 'rgba(255,255,255,0.7)', fontSize: 12 }}>搜索商品</div>
          )}
        </div>
      );

    case 'z_auxiliary_box':
      return <div style={{ height: cfg.height || 20, background: cfg.bgColor || '#f5f5f5' }} />;

    case 'z_auxiliary_line':
      return (
        <div style={{ padding: `${cfg.margin?.top || 0}px ${cfg.padding?.right || 0}px ${cfg.margin?.bottom || 0}px ${cfg.padding?.left || 0}px` }}>
          <div style={{ borderTop: `${cfg.lineHeight || 1}px ${cfg.lineStyle || 'solid'} ${cfg.lineColor || '#eee'}` }} />
        </div>
      );

    case 'z_ueditor':
      return (
        <div style={{ background: cfg.bgColor || '#fff', padding: `${cfg.padding?.top || 10}px ${cfg.padding?.right || 12}px ${cfg.padding?.bottom || 10}px ${cfg.padding?.left || 12}px` }}>
          <div dangerouslySetInnerHTML={{ __html: cfg.content || '<p>富文本内容</p>' }} style={{ fontSize: 13, lineHeight: 1.6 }} />
        </div>
      );
    case 'home_goods_list':
      return (
        <div style={{ background: cfg.bgColor || '#f5f5f5', padding: `${cfg.padding?.top || 10}px ${cfg.padding?.right || 10}px ${cfg.padding?.bottom || 10}px ${cfg.padding?.left || 10}px` }}>
          <div style={{ display: 'grid', gridTemplateColumns: cfg.listStyle === 'list' ? '1fr' : `repeat(${cfg.columns || 2}, 1fr)`, gap: 8 }}>
            {[1, 2, 3, 4].map((i) => (
              <div key={i} style={{ background: cfg.cardBgColor || '#fff', borderRadius: cfg.borderRadius || 8, overflow: 'hidden' }}>
                <div style={{ height: cfg.listStyle === 'list' ? 80 : 120, background: '#f0f0f0', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#ccc', fontSize: 12 }}>商品图</div>
                <div style={{ padding: 8 }}>
                  {cfg.showName !== false && <div style={{ fontSize: 12, color: '#333' }}>商品名称</div>}
                  {cfg.showPrice !== false && <div style={{ fontSize: 13, color: '#e93323', fontWeight: 600 }}>¥99.00</div>}
                </div>
              </div>
            ))}
          </div>
        </div>
      );

    case 'home_coupon':
      return (
        <div style={{ background: cfg.bgColor || '#fff', padding: `${cfg.padding?.top || 10}px ${cfg.padding?.right || 12}px ${cfg.padding?.bottom || 10}px ${cfg.padding?.left || 12}px` }}>
          <div style={{ display: 'flex', gap: 8, overflow: 'hidden' }}>
            {[1, 2, 3].map((i) => (
              <div key={i} style={{ minWidth: 100, background: cfg.primaryColor || '#e93323', borderRadius: 6, padding: '8px 12px', color: '#fff', textAlign: 'center' }}>
                <div style={{ fontSize: 18, fontWeight: 700 }}>¥{i * 10}</div>
                <div style={{ fontSize: 10, opacity: 0.8 }}>满{i * 100}可用</div>
              </div>
            ))}
          </div>
        </div>
      );

    case 'home_bargain':
    case 'home_seckill':
    case 'home_group': {
      const labels: Record<string, string> = { home_bargain: '砍价', home_seckill: '秒杀', home_group: '拼团' };
      return (
        <div style={{ background: cfg.bgColor || '#fff', padding: `${cfg.padding?.top || 10}px ${cfg.padding?.right || 12}px ${cfg.padding?.bottom || 10}px ${cfg.padding?.left || 12}px` }}>
          <div style={{ fontSize: 14, fontWeight: 600, color: cfg.titleColor || '#e93323', marginBottom: 8 }}>{labels[comp.name]}专区</div>
          <div style={{ display: 'flex', gap: 8, overflow: 'hidden' }}>
            {[1, 2, 3].map((i) => (
              <div key={i} style={{ minWidth: 100, background: '#f9f9f9', borderRadius: 6, overflow: 'hidden' }}>
                <div style={{ height: 80, background: '#f0f0f0', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#ccc', fontSize: 11 }}>商品图</div>
                <div style={{ padding: 6, textAlign: 'center' }}>
                  <div style={{ fontSize: 13, color: cfg.priceColor || '#e93323', fontWeight: 600 }}>¥{i * 9}.9</div>
                </div>
              </div>
            ))}
          </div>
        </div>
      );
    }
    case 'home_article':
      return (
        <div style={{ background: cfg.bgColor || '#fff', padding: `${cfg.padding?.top || 10}px ${cfg.padding?.right || 12}px ${cfg.padding?.bottom || 10}px ${cfg.padding?.left || 12}px` }}>
          {[1, 2].map((i) => (
            <div key={i} style={{ display: 'flex', gap: 10, padding: '8px 0', borderBottom: '1px solid #f5f5f5' }}>
              <div style={{ width: 80, height: 60, background: '#f0f0f0', borderRadius: 4, flexShrink: 0, display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#ccc', fontSize: 10 }}>图片</div>
              <div>
                <div style={{ fontSize: 13, color: cfg.titleColor || '#333', fontWeight: 500 }}>文章标题 {i}</div>
                <div style={{ fontSize: 11, color: '#999', marginTop: 4 }}>文章摘要内容...</div>
              </div>
            </div>
          ))}
        </div>
      );

    case 'home_video':
      return (
        <div style={{ background: cfg.bgColor || '#000', padding: `${cfg.padding?.top || 0}px ${cfg.padding?.right || 0}px ${cfg.padding?.bottom || 0}px ${cfg.padding?.left || 0}px` }}>
          <div style={{ height: 200, background: '#1a1a1a', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#fff', fontSize: 32 }}>▶</div>
        </div>
      );

    case 'home_tab':
      return (
        <div style={{ background: cfg.bgColor || '#fff' }}>
          <div style={{ display: 'flex', borderBottom: '2px solid #f0f0f0' }}>
            {(cfg.tabList || []).map((tab: any, i: number) => (
              <div key={i} style={{ flex: 1, textAlign: 'center', padding: '8px 0', fontSize: 13, color: i === 0 ? (cfg.activeColor || '#e93323') : (cfg.inactiveColor || '#333'), borderBottom: i === 0 ? `2px solid ${cfg.activeColor || '#e93323'}` : 'none', fontWeight: i === 0 ? 600 : 400 }}>
                {tab.title}
              </div>
            ))}
          </div>
          <div style={{ padding: 12, color: '#999', fontSize: 12, textAlign: 'center' }}>选项卡内容区域</div>
        </div>
      );

    case 'home_news_roll':
      return (
        <div style={{ background: cfg.bgColor || '#fffbe6', padding: `${cfg.padding?.top || 8}px ${cfg.padding?.right || 12}px ${cfg.padding?.bottom || 8}px ${cfg.padding?.left || 12}px`, display: 'flex', alignItems: 'center', gap: 8 }}>
          <span style={{ color: cfg.iconColor || '#faad14', fontSize: 14 }}>📢</span>
          <span style={{ fontSize: 12, color: cfg.textColor || '#faad14' }}>{cfg.newsList?.[0]?.text || '滚动消息'}</span>
        </div>
      );
    case 'home_hotspot':
      return (
        <div style={{ padding: `${cfg.padding?.top || 0}px ${cfg.padding?.right || 0}px ${cfg.padding?.bottom || 0}px ${cfg.padding?.left || 0}px` }}>
          <div style={{ height: 150, background: cfg.bgImg ? `url(${cfg.bgImg}) center/cover` : '#f0f0f0', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#999', fontSize: 13 }}>
            {!cfg.bgImg && '热区 (请上传背景图)'}
          </div>
        </div>
      );

    case 'picture_cube': {
      const cols = parseInt(cfg.layout) || 2;
      return (
        <div style={{ padding: `${cfg.padding?.top || 0}px ${cfg.padding?.right || 0}px ${cfg.padding?.bottom || 0}px ${cfg.padding?.left || 0}px` }}>
          <div style={{ display: 'grid', gridTemplateColumns: `repeat(${cols}, 1fr)`, gap: cfg.gap || 4 }}>
            {(cfg.imgList || []).map((_: any, i: number) => (
              <div key={i} style={{ height: 100, background: '#f0f0f0', borderRadius: cfg.borderRadius || 0, display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#ccc', fontSize: 11 }}>图片{i + 1}</div>
            ))}
          </div>
        </div>
      );
    }

    case 'home_merchant':
      return (
        <div style={{ background: cfg.bgColor || '#fff', padding: `${cfg.padding?.top || 10}px ${cfg.padding?.right || 12}px ${cfg.padding?.bottom || 10}px ${cfg.padding?.left || 12}px`, display: 'flex', alignItems: 'center', gap: 10 }}>
          {cfg.showLogo && <div style={{ width: 48, height: 48, borderRadius: 8, background: '#f0f0f0', flexShrink: 0, display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#ccc', fontSize: 10 }}>Logo</div>}
          <div>
            {cfg.showName && <div style={{ fontSize: 14, fontWeight: 600, color: '#333' }}>店铺名称</div>}
            {cfg.showDesc && <div style={{ fontSize: 12, color: '#999', marginTop: 2 }}>店铺描述信息</div>}
          </div>
        </div>
      );

    default:
      return (
        <div style={{ padding: 12, background: '#fafafa', textAlign: 'center', color: '#999', fontSize: 12 }}>
          {comp.cname} 组件
        </div>
      );
  }
};

export default ComponentPreview;
