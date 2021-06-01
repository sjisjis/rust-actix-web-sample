DROP TABLE IF EXISTS `user`;
CREATE TABLE `user` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(30) NOT NULL COMMENT 'アカウント名',
  `mailadress` varchar(255) NOT NULL COMMENT 'ユーザメール',
  `password` varchar(30) DEFAULT NULL COMMENT 'パスワード',
  `created_at` datetime NOT NULL COMMENT '作成日',
  `updated_at` datetime NOT NULL COMMENT '更新日',
  `deleted_at` datetime DEFAULT NULL COMMENT '削除日',
  PRIMARY KEY (`id`),
  UNIQUE KEY `mailadress` (`mailadress`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

