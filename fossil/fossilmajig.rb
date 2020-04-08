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
    userdata = User.first username: session[:user_id]
    if userdata.nil?
      erb :login, :locals => { :nouser => true }
    else
      username = userdata.username
      owned = binary_to_array(userdata.owned)
      erb :main, :locals => { :fossils => fossils, :username => username, :owned => owned, :raw => userdata.owned }
    end
  end

  get '/login' do
    erb :login, :locals => { :nouser => false }
  end

  post '/savedata' do
    output = params.values.join('')
    userdata = User.first username: session[:user_id]
    userdata.owned = output
    userdata.save
    redirect '/'
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
    retval.unshift("0")
  end

  def array_to_binary(arr)
    arr.join
  end

end
