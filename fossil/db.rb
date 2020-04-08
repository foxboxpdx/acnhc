require 'dm-core'

DataMapper.setup(:default, "sqlite://#{Dir.pwd}/database.db")

class User
  include DataMapper::Resource
  property :id, Serial
  property :username, String, :length=>50, :unique=>true, :required=>true
  property :owned, String, :length=>73, :required=>true
  property :extra, String, :length=>73, :required=>true
end

class Fossil
  include DataMapper::Resource
  property :id, Serial
  property :name, String, :length=>50, :unique=>true, :required=>true
end

DataMapper.finalize
