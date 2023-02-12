# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: article.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import builder as _builder
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\rarticle.proto\x12\x07\x61rticle\"\xaf\x01\n\x0e\x41rticleMessage\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x0c\n\x04site\x18\x02 \x01(\t\x12\x0b\n\x03url\x18\x03 \x01(\t\x12\x10\n\x08language\x18\x04 \x01(\t\x12\x13\n\x0bscrape_date\x18\x05 \x01(\t\x12\x1c\n\x0fsubmission_date\x18\x06 \x01(\tH\x00\x88\x01\x01\x12\x1b\n\x13tags_and_categories\x18\x07 \x03(\tB\x12\n\x10_submission_date\"\x18\n\x16ReadArticleListRequest\"9\n\"ReadArticleListBySearchtermRequest\x12\x13\n\x0bsearch_term\x18\x01 \x01(\t\",\n\x1cReadArticleListBySiteRequest\x12\x0c\n\x04site\x18\x01 \x01(\t\"Q\n\x17ReadArticleListResponse\x12)\n\x08\x61rticles\x18\x01 \x03(\x0b\x32\x17.article.ArticleMessage\x12\x0b\n\x03msg\x18\x02 \x01(\t\"]\n#ReadArticleListBySearchtermResponse\x12)\n\x08\x61rticles\x18\x01 \x03(\x0b\x32\x17.article.ArticleMessage\x12\x0b\n\x03msg\x18\x02 \x01(\t\"W\n\x1dReadArticleListBySiteResponse\x12)\n\x08\x61rticles\x18\x01 \x03(\x0b\x32\x17.article.ArticleMessage\x12\x0b\n\x03msg\x18\x02 \x01(\t2\xce\x02\n\x0e\x41rticleService\x12V\n\x0fReadArticleList\x12\x1f.article.ReadArticleListRequest\x1a .article.ReadArticleListResponse\"\x00\x12z\n\x1bReadArticleListBySearchterm\x12+.article.ReadArticleListBySearchtermRequest\x1a,.article.ReadArticleListBySearchtermResponse\"\x00\x12h\n\x15ReadArticleListBySite\x12%.article.ReadArticleListBySiteRequest\x1a&.article.ReadArticleListBySiteResponse\"\x00\x62\x06proto3')

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, globals())
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'article_pb2', globals())
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _ARTICLEMESSAGE._serialized_start=27
  _ARTICLEMESSAGE._serialized_end=202
  _READARTICLELISTREQUEST._serialized_start=204
  _READARTICLELISTREQUEST._serialized_end=228
  _READARTICLELISTBYSEARCHTERMREQUEST._serialized_start=230
  _READARTICLELISTBYSEARCHTERMREQUEST._serialized_end=287
  _READARTICLELISTBYSITEREQUEST._serialized_start=289
  _READARTICLELISTBYSITEREQUEST._serialized_end=333
  _READARTICLELISTRESPONSE._serialized_start=335
  _READARTICLELISTRESPONSE._serialized_end=416
  _READARTICLELISTBYSEARCHTERMRESPONSE._serialized_start=418
  _READARTICLELISTBYSEARCHTERMRESPONSE._serialized_end=511
  _READARTICLELISTBYSITERESPONSE._serialized_start=513
  _READARTICLELISTBYSITERESPONSE._serialized_end=600
  _ARTICLESERVICE._serialized_start=603
  _ARTICLESERVICE._serialized_end=937
# @@protoc_insertion_point(module_scope)