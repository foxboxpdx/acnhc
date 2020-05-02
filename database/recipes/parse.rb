
fname = ARGV.shift
data = File.read(fname).split("\n")
flag = 0
recipes = Array.new()

data.each do |line|
  if line.chomp.eql?("|-")
      flag = 1
      next
  end
  if flag.eql?(1)
      foo = line.chomp
      foo.gsub!(/\|\[\[/, '')
      foo.gsub!(/\]\].*$/, '')
      recipes.push(foo.split('|').pop)
      flag = 0
  end
end

out = recipes.join("\n")
outfile = fname.gsub(/.txt/, '.fix')
open(outfile, 'w') { |f|
  f.puts out
}
