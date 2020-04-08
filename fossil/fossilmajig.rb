require 'sinatra/base'
require 'data_mapper'
require 'securerandom'
require_relative 'db'

class FossilMajig < Sinatra::Base

  set :root, File.dirname(__FILE__)

  use Rack::Session::Cookie, :key => 'rack.session',
                             :path => '/',
                             :secret => 'benchdtails'

  def needs_auth
    return !session[:user_id]
  end

  get '/' do
    redirect '/login' if needs_auth
    fossils = Fossil.all
    userdata = Site.username(session[:user_id])
    username = userdata.username
    owned = binary_to_array(userdata.owned)
    extra = binary_to_array(userdata.extra)

    erb :main, :locals => { :fossils => fossils, :username => username, :owned => owned, :extra => extra }
  end

  get '/login' do
    erb :login
  end

  post '/newuser' do
    fossils = "0" * 73
    username = SecureRandom.uuid
    foo = User.new username: username, owned: fossils, extra: fossils
    foo.save
    session[:user_id] = username
    redirect '/'
  end

  post '/loggedin' do
    session[:user_id] = params["user_id"]
    redirect '/'
  end

  get '/logout' do
    session.clear
    redirect '/'
  end

  def binary_to_array(string)
    retval = string.split(//)
    if retval.length.eql?(0)
        foo = "0" * 73
        retval = foo.split(//)
    end
    retval
  end

  def array_to_binary(arr)
    arr.join
  end

end
