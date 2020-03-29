require 'dm-core'

DataMapper.setup(:default, "sqlite://acnh.db")

class Furniture
    include DataMapper::Resource
    property :id, Serial
    property :name, String, :length=>256, :unique=>true, :required=>true
    property :length, Float, :required=>true, :default=> 1
    property :width, Float, :required=>true, :default=>1
    property :floor, Boolean, :required=>true, :default=>true
    property :wall, Boolean, :required=>true, :default=>true
    property :sellprice, String, :length=>12, :required=>true, :default=>"Cannot Sell"
    property :buyprice, String, :length=>12, :required=>true, :default=>"Cannot Buy"

    has n, :furnitureVariants
end

class FurnitureVariant
    include DataMapper::Resource
    property :id, Serial
    property :name, String, :length=>256, :required=>true

    belongs_to :furniture, :key => true
end

class Sahara
    include DataMapper::Resource
    property :id, Serial
    property :name, String, :length=>256, :unique=>true, :required=>true
    # True if this is a wall
    property :wall, Boolean, :required=>true, :default=>true
    # 0=full 1=small 2=medium 3=large
    property :size, Integer, :required=>true, :default=>0
    property :animated, Boolean, :required=>true, :default=>false
    property :sellprice, String, :length=>12, :required=>true, :default=>"Cannot Sell"
    property :buyprice, String, :length=>12, :required=>true, :default=>"Cannot Buy"
end

class Flower
    include DataMapper::Resource
    property :id, Serial
    property :species, String, :length=>256, :required=>true, :key=>true
    property :color, String, :length=>256, :required=>true, :key=>true
    property :hybrid, Boolean, :required=>true, :default=>false
    property :sellprice, Integer, :required=>true, :default=>0
end

class Fish
    include DataMapper::Resource
    property :id, Serial
    property :name, String, :length=>256, :required=>true, :unique=>true
    property :river, Boolean, :required=>true, :default=>false
    property :sea, Boolean, :required=>true, :default=>false
    property :rivermouth, Boolean, :required=>true, :default=>false
    property :elevation, Boolean, :required=>true, :default=>false
    property :rain, Boolean, :required=>true, :default=>false
    property :months, String, :length=>256, :required=>true, :default=>"Year Round"
    property :hours, String, :length=>256, :required=>true, :default=>"All day"
    property :sellprice, Integer, :required=>true, :default=>0
end

class Bug
    include DataMapper::Resource
    property :id, Serial
    property :name, String, :length=>256, :required=>true, :unique=>true
    property :rain, Boolean, :required=>true, :default=>false
    property :stump, Boolean, :required=>true, :default=>false
    property :rock, Boolean, :required=>true, :default=>false
    property :tree, Boolean, :required=>true, :default=>false
    property :special, String, :length=>256, :default=>"None"
    property :months, String, :length=>256, :required=>true, :default=>"Year Round"
    property :hours, String, :length=>256, :required=>true, :default=>"All day"
    property :sellprice, Integer, :required=>true, :default=>0
end

class Fossil
    include DataMapper::Resource
    property :id, Serial
    property :name, String, :length=>256, :required=>true, :unique=>true
    property :sellprice, Integer, :required=>true, :default=>0
end

class Clothing
    include DataMapper::Resource
    property :id, Serial
    property :name, String, :length=>256, :required=>true, :unique=>true
    property :ctype, String, :length=>50, :required=>true, :default=>"Hat"    
    property :sellprice, String, :length=>12, :required=>true, :default=>"Cannot Sell"
    property :buyprice, String, :length=>12, :required=>true, :default=>"Cannot Buy"

    has n, :clothingVariants
end

class ClothingVariant
    include DataMapper::Resource
    property :id, Serial
    property :name, String, :length=>256, :required=>true

    belongs_to :clothing, :key => true
end

class Recipe
    include DataMapper::Resource
    property :id, Serial
    property :name, String, :length=>256, :required=>true, :unique=>true
    property :rtype, String, :length=>50, :required=>true, :default=>"Tool"
end

class Item
    include DataMapper::Resource
    property :id, Serial
    property :name, String, :length=>256, :required=>true, :unique=>true
    property :sellprice, Integer, :required=>true, :default=>0
end
