DROP TABLE IF EXISTS `article`;
CREATE TABLE `article`
(
    `id`            int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
    `class_id`      int(10) DEFAULT 0 COMMENT '类别ID',
    `name`          varchar(100) NOT NULL COMMENT '文章名',
    `create_at`     datetime NOT NULL COMMENT '创建时间',
    `location`      text DEFAULT NULL COMMENT '文章位置',
    `come_from`     varchar(100) DEFAULT NULL COMMENT '文章来源',
    `modify_at`     datetime DEFAULT NULL COMMENT '修改时间',
    `audit_at`      datetime DEFAULT NULL COMMENT '审核时间',
    `label_id_list` varchar(100) DEFAULT NULL COMMENT '标签id列表',
    `last_img`      text DEFAULT NULL COMMENT '最新图片',
    `sum_txt`       varchar(300) DEFAULT NULL COMMENT '摘要描述',
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=UTF8MB4;

DROP TABLE IF EXISTS `class`;
CREATE TABLE `class`
(
    `id`            int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
    `class_name`    varchar(45) NOT NULL COMMENT '分类名',
    `detail`        varchar(300) DEFAULT NULL COMMENT '分类描述',
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=UTF8MB4;

DROP TABLE IF EXISTS `label`;
CREATE TABLE `label`
(
    `id`            int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
    `label_name`    varchar(45) NOT NULL COMMENT '标签名',
    `detail`        varchar(300) DEFAULT NULL COMMENT '标签描述',
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=UTF8MB4;